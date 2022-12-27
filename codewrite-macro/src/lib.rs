use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

fn try_parse_expr(
    tokens: &[TokenTree],
    remove_separator: bool,
) -> Option<(&[TokenTree], &[TokenTree])> {
    let mut iter = tokens.into_iter().enumerate();
    while let Some((i, next)) = iter.next() {
        match next {
            TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => continue,
            TokenTree::Punct(p) => {
                if matches!(p.as_char(), ',' | ';' | ':') {
                    if remove_separator {
                        return Some((&tokens[..i], &tokens[i + 1..]));
                    } else {
                        return Some((&tokens[..i], &tokens[i..]));
                    }
                }
            }
        }
    }

    Some((tokens, &[]))
}

#[derive(Clone)]
struct WhitespaceEater<I: Iterator<Item = char>>(I, bool);
impl<I: Iterator<Item = char>> Iterator for WhitespaceEater<I> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let WhitespaceEater(iter, last_whitespace) = self;
        while let Some(next) = iter.next() {
            let whitespace = next.is_ascii_whitespace();
            match (*last_whitespace, whitespace) {
                (true, true) => continue,
                (false, true) => return Some(' '),
                _ => {
                    *last_whitespace = false;
                    return Some(next);
                }
            }
        }
        None
    }
}

#[proc_macro]
pub fn code(item: TokenStream) -> TokenStream {
    let tokens = item.into_iter().collect::<Vec<_>>();
    let mut slice = tokens.as_slice();

    if slice.is_empty() {
        return TokenStream::new();
    }

    let (buf, rem) = try_parse_expr(slice, true)
        .expect("Failed to parse 'buf' expression at the start of macro");
    slice = rem;
    let (ctx, rem) = try_parse_expr(slice, true)
        .expect("Failed to parse 'ctx' expression at the start of macro");
    slice = rem;

    if slice.is_empty() {
        return TokenStream::new();
    }

    let mut interpolations: Vec<TokenTree> = Vec::new();

    let out_tokens = visit_ast_nodes(slice, &mut interpolations);

    let str_string = {
        let str_temp = TokenStream::from_iter(out_tokens.into_iter())
            .to_string()
            // replace the previous interpolation marker with the final null byte that is consumed by the runtime machinery
            .replace("\"~~__interp_marker\"", "\0");
        let str_chars = WhitespaceEater(str_temp.chars(), true);

        let mut chars = str_chars.collect::<Vec<_>>();
        let mut out = String::new();

        // really wasteful algorithm, but I'm actually able to think about it
        // some linear scan state machine should be possible
        for i in 0..chars.len() {
            let c0 = chars.get(i.overflowing_sub(1).0).cloned().unwrap_or(' ');
            let c1 = chars[i];
            let c2 = chars.get(i + 1).cloned().unwrap_or(' ');

            if c1 == ' ' && (!c0.is_ascii_alphanumeric() || !c2.is_ascii_alphanumeric()) {
                // we have skipped this character, overwrite this char with something not-whitespace to update the situation for the following char
                chars[i] = 'a';
            } else {
                out.push(c1);
            }
        }

        out
    };

    let str = TokenTree::Literal(Literal::string(&str_string));
    let buf = TokenTree::Group(Group::new(
        Delimiter::None,
        TokenStream::from_iter(buf.iter().cloned()),
    ));
    let ctx = TokenTree::Group(Group::new(
        Delimiter::None,
        TokenStream::from_iter(ctx.iter().cloned()),
    ));

    let interp = TokenTree::Group(Group::new(
        Delimiter::Bracket,
        interpolations
            .into_iter()
            .map(|s| {
                [
                    TokenTree::Punct(Punct::new('&', Spacing::Alone)),
                    s,
                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                ]
            })
            .flatten()
            .collect(),
    ));

    format!(
        "
        {{
            use codewrite::write::InterpWriter;
            use std::borrow::BorrowMut;
            {buf}.write_with_interpolations(
                {ctx}.borrow_mut(),
                {str},
                &{interp}
            );
        }}
    "
    )
    .parse()
    .expect("Failed to re-parse the output string")
}

fn group_tokens(tokens: &[TokenTree]) -> TokenTree {
    TokenTree::Group(Group::new(
        Delimiter::None,
        TokenStream::from_iter(tokens.iter().cloned()),
    ))
}

fn visit_ast_nodes(mut slice: &[TokenTree], interpolations: &mut Vec<TokenTree>) -> TokenStream {
    let mut out_tokens: Vec<TokenTree> = Vec::new();
    loop {
        match slice {
            // escaped raw '$'
            [TokenTree::Punct(p1), TokenTree::Punct(p2), ..]
                if p1.as_char() == '\\' && p2.as_char() == '$' =>
            {
                out_tokens.push(slice[1].clone());
                slice = &slice[2..];
            }
            [TokenTree::Punct(p1), ..] if p1.as_char() == '$' => {
                match &slice[1..] {
                    // macro call, TODO support path qualified macros
                    [TokenTree::Ident(_), TokenTree::Punct(p2), TokenTree::Group(_), ..]
                        if p2.as_char() == '!' =>
                    {
                        interpolations.push(group_tokens(&slice[1..4]));
                        slice = &slice[4..];
                    }
                    // normal ident
                    [TokenTree::Ident(_), ..] => {
                        interpolations.push(group_tokens(&slice[1..2]));
                        slice = &slice[2..];
                    }
                    // parenthesized expression
                    [TokenTree::Group(_), ..] => {
                        interpolations.push(group_tokens(&slice[1..2]));
                        slice = &slice[2..];
                    }
                    _ => panic!("Unsupported syntax after '$' ({:?})", p1.span()),
                }
                // something easy to search for that is hopefully unique
                out_tokens.push(TokenTree::Literal(Literal::string("~~__interp_marker")));
            }
            [TokenTree::Group(g), ..] => {
                let group_tokens = g.stream().into_iter().collect::<Vec<_>>();
                let group_stream = visit_ast_nodes(&group_tokens, interpolations);

                out_tokens.push(TokenTree::Group(Group::new(g.delimiter(), group_stream)));
                slice = &slice[1..];
            }
            [_, ..] => {
                out_tokens.push(slice[0].clone());
                slice = &slice[1..];
            }
            [] => break,
        }
    }
    TokenStream::from_iter(out_tokens.into_iter())
}
