use proc_macro::{TokenStream, TokenTree};

fn parse_expr(
    iter: &mut std::iter::Peekable<proc_macro::token_stream::IntoIter>,
) -> Vec<TokenTree> {
    let mut expr = Vec::new();

    loop {
        let next = iter.peek().unwrap();
        match next {
            TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {}
            TokenTree::Punct(_) => panic!("Unexpected punctuation '{}'", next),
        };
        expr.push(next.clone());
        iter.next();

        if let Some(next) = iter.next() {
            match &next {
                TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {
                    panic!("Unexpected tty when parsing punctuation '{}'", next)
                }
                TokenTree::Punct(p) => {
                    if matches!(p.as_char(), ',' | ';') {
                        break;
                    }
                }
            }
            expr.push(next.clone());
        } else {
            break;
        }
    }

    expr
}

#[derive(PartialEq, Eq)]
enum State {
    Free,
    Wrapped,
    IdentDone,
}

enum Chunk {
    Literal(String),
    Expr(String),
}

#[proc_macro]
pub fn code(item: TokenStream) -> TokenStream {
    let mut iter: std::iter::Peekable<proc_macro::token_stream::IntoIter> =
        item.into_iter().peekable();

    if iter.peek().is_none() {
        return TokenStream::new();
    }

    let buf = TokenStream::from_iter(parse_expr(&mut iter));
    let mut chunks = Vec::new();

    let _src = TokenStream::from_iter(iter).to_string();
    let mut src = _src.chars().peekable();
    let mut str = String::new();

    'outer: loop {
        let next = src.next();
        match next {
            None => {}
            Some('#') => {
                loop {
                    match src.peek() {
                        // the compiler may inject whitespace anywhere
                        Some(w) if w.is_whitespace() => {}
                        Some('#') => {
                            str.push('#');
                            src.next();
                            continue 'outer;
                        }
                        _ => break,
                    }
                    src.next();
                }
            }
            Some(c) => {
                str.push(c);
                continue;
            }
        }
        if !str.is_empty() {
            chunks.push(Chunk::Literal(str.trim().to_owned()));
            str.clear();
        }
        if next == Some('#') {
            let mut scope_count: u32 = 0;
            let mut first = true;
            let mut state = State::Free;

            while let Some(&next) = src.peek() {
                match next {
                    '{' | '(' | '[' => {
                        scope_count += 1;
                        if first || (next == '(' && state == State::IdentDone) {
                            state = State::Wrapped;
                        } else if state != State::Wrapped {
                            break;
                        }
                    }
                    '}' | ')' | ']' => {
                        if state == State::Free || scope_count == 0 {
                            break;
                        }

                        scope_count = scope_count.checked_sub(1).expect("Unbalanced group");

                        if scope_count == 0 {
                            if state == State::Wrapped {
                                src.next();
                                str.push(next);
                                break;
                            }
                        }
                    }
                    _ if next.is_whitespace() || next == '!' => {
                        if state == State::Free {
                            state = State::IdentDone;
                        }
                    }
                    _ if state == State::IdentDone => {
                        break;
                    }
                    _ => {}
                }
                first = false;
                if next.is_ascii_alphanumeric()
                    || next == '_'
                    || matches!(state, State::Wrapped | State::IdentDone)
                {
                    src.next();
                    str.push(next);
                } else {
                    // panic!("'{}'", next);
                    break;
                }
            }
            assert!(!str.is_empty());
            chunks.push(Chunk::Expr(str.trim().to_owned()));
            str.clear();
        } else {
            break;
        }
    }

    use std::fmt::Write;

    writeln!(str, "{{").unwrap();
    writeln!(str, "use std::borrow::BorrowMut;").unwrap();
    writeln!(str, "let buf = ({buf}).borrow_mut();").unwrap();

    for chunk in chunks {
        writeln!(str, "buf.separate_idents();").unwrap();
        match chunk {
            Chunk::Literal(s) => {
                const PAD: &str = "###";
                let s = s.trim();
                writeln!(str, "buf.write_str(r{PAD}\"{}\"{PAD}).unwrap();", s).unwrap();
            }
            Chunk::Expr(s) => {
                let s = s.trim();
                writeln!(str, "{s}.efmt(buf).unwrap();").unwrap();
            }
        }
    }
    writeln!(str, "}}").unwrap();

    str.parse().expect("Failed to re-parse the output string")
}
