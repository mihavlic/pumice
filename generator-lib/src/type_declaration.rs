use std::{
    fmt::{Debug, Display, Write},
    iter::Peekable,
    marker::PhantomData,
    slice,
};

use crate::{interner::UniqueStr, Intern, Interner};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<'a> {
    Alphanumeric(&'a str),
    Ptr,
    Const,
    LBracket,
    RBracket,
    LParen,
    RParen,
    DoubleColon,
}

pub struct CLexer<'a> {
    cur: *const u8,
    end: *const u8,
    ident: Option<*const u8>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> CLexer<'a> {
    pub fn new(str: &'a str) -> Self {
        unsafe {
            Self {
                cur: str.as_ptr(),
                end: str.as_ptr().add(str.len()),
                ident: None,
                phantom: PhantomData,
            }
        }
    }
}

impl<'a> Iterator for CLexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe fn str_from_ptr_range(start: *const u8, end: *const u8) -> &'static str {
            let bytes = slice::from_raw_parts(start, end.offset_from(start) as usize);
            std::str::from_utf8_unchecked(bytes)
        }

        let Self {
            cur, end, ident, ..
        } = self;

        loop {
            unsafe {
                if cur == end {
                    return None;
                }

                let char = str_from_ptr_range(*cur, *end)
                    .chars()
                    .next()
                    .unwrap_unchecked();
                let next = cur.add(char.len_utf8());
                let alphanumeric = char.is_ascii_alphanumeric() || char == '_';

                if alphanumeric {
                    if ident.is_none() {
                        // this character is potentially the start of a multi-character token
                        *ident = Some(*cur);
                    }
                    *cur = next;
                }

                // we're either on the last character or the current ident has ended
                if !alphanumeric || next == *end {
                    if let Some(ident) = ident.take() {
                        let ident = str_from_ptr_range(ident, *cur);
                        match ident {
                            // struct in type declaration is ignored
                            "struct" => continue,
                            "const" => return Some(Token::Const),
                            _ => return Some(Token::Alphanumeric(ident)),
                        };
                    } else {
                        // note that we increment after this character only after we've potentially returned an ident
                        // looped around, and now we're actually handling the single-character token
                        *cur = next;
                        match char {
                            '*' => return Some(Token::Ptr),
                            '[' => return Some(Token::LBracket),
                            ']' => return Some(Token::RBracket),
                            '(' => return Some(Token::LParen),
                            ')' => return Some(Token::RParen),
                            ':' => return Some(Token::DoubleColon),
                            _ => continue,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyToken {
    BaseType(UniqueStr),
    Ptr,
    Array(Option<UniqueStr>),
    Const,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Type(pub Vec<TyToken>);

impl Type {
    pub fn try_only_basetype(&self) -> Option<UniqueStr> {
        // type cannot be empty, unwrapping is fine
        if let &TyToken::BaseType(s) = self.0.get(0).unwrap() {
            Some(s)
        } else {
            None
        }
    }
    pub fn from_ty_tokens(tokens: Vec<TyToken>) -> Self {
        Self(tokens)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_type_tokens_simple(&self.0, f)
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        fmt_type_tokens_simple(&self.0, f)?;
        f.write_char('"')
    }
}

fn fmt_type_tokens_simple(tokens: &[TyToken], f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let resolve = |s: UniqueStr, f: &mut std::fmt::Formatter| f.write_str(s.resolve());
    fmt_type_tokens_impl(tokens, &resolve, &resolve, f)
}

pub fn fmt_type_tokens_impl<
    F1: Fn(UniqueStr, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
    F2: Fn(UniqueStr, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
>(
    tokens: &[TyToken],
    on_ident: &F1,
    on_arr: &F2,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    for (i, token) in tokens.iter().enumerate() {
        match token {
            &TyToken::BaseType(s) => {
                on_ident(s, f)?;
            }
            TyToken::Ptr => {
                f.write_char('*')?;

                if !matches!(tokens.get(i + 1), Some(&TyToken::Const)) {
                    f.write_str("mut ")?;
                }
            }
            &TyToken::Array(s) => {
                f.write_char('[')?;
                fmt_type_tokens_impl(&tokens[i + 1..], on_ident, on_arr, f)?;
                if let Some(size) = s {
                    f.write_str("; ")?;
                    on_arr(size, f)?;
                }
                f.write_char(']')?;
                return Ok(());
            }
            TyToken::Const => {
                f.write_str("const")?;
            }
        }

        if i != tokens.len() - 1 && *token != TyToken::Ptr {
            f.write_char(' ')?;
        }
    }
    Ok(())
}

pub struct Lexer<'a>(Peekable<CLexer<'a>>);

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self(CLexer::new(str).peekable())
    }
    pub fn next(&mut self) -> Option<Token> {
        self.0.next()
    }
    pub fn peek(&mut self) -> Option<Token> {
        self.0.peek().cloned()
    }
    pub fn expect(&mut self, tok: Token) {
        let next = self.next();
        assert!(Some(tok) == next, "Expected '{:?}', got {:?}", tok, next);
    }
    pub fn consume_all(&mut self, tok: Token) -> usize {
        let mut i = 0;
        while let Some(next) = self.peek() {
            if next == tok {
                i += 1;
                self.next();
            } else {
                return i;
            }
        }
        return i;
    }
    pub fn consume_n(&mut self, tok: Token, n: usize) -> usize {
        let mut i = 0;
        while let Some(next) = self.peek() {
            if next == tok {
                i += 1;
                self.next();

                if i == n {
                    return n;
                }
            } else {
                break;
            }
        }

        return i;
    }
    pub fn consume_all_fn<F: Fn(Token) -> bool>(&mut self, fun: F) -> usize {
        let mut i = 0;
        while let Some(next) = self.peek() {
            if !fun(next) {
                break;
            }
            i += 1;
        }
        return i;
    }
}

// Bloody hell, C type decls are such a mess!
// Resources used:
//   https://eli.thegreenplace.net/2008/07/18/reading-c-type-declarations/
//   http://www.unixwiz.net/techtips/reading-cdecl.html
//   https://cdecl.org/
// TODO support function pointers
pub fn parse_type_decl(str: &str, int: &Interner) -> (Option<UniqueStr>, Type, Option<u8>) {
    let mut l = Lexer::new(str);
    let parens = l.consume_all(Token::LParen);

    let mut tokens = Vec::new();
    match l.next().unwrap() {
        Token::Alphanumeric(s) => {
            tokens.push(TyToken::BaseType(s.intern(int)));
            if let Some(Token::Const) = l.peek() {
                tokens.push(TyToken::Const);
                l.next();
            }
        }
        Token::Const => {
            if let Some(Token::Alphanumeric(s)) = l.next() {
                tokens.push(TyToken::BaseType(s.intern(int)));
                tokens.push(TyToken::Const);
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    };

    let mut name = None;
    let mut bitfield = None;
    recursive_parse(&mut l, &mut tokens, &mut name, &mut bitfield, int);

    l.consume_n(Token::RParen, parens);

    // reverse the order of all consecutive `TyToken::Array`s since they should be parsed in the reverse direction that
    // the modifiers left of the variable name
    let mut i = 0;
    while i < tokens.len() {
        if let TyToken::Array(_) = tokens[i] {
            let start = i;
            i += 1;
            while i < tokens.len() {
                if !matches!(tokens[i], TyToken::Array(_)) {
                    break;
                }
                i += 1;
            }

            tokens[start..i].reverse();
        }
        i += 1;
    }

    // we've been parsing such that the basetype is at the start and the type is built off it
    // it's really more useful to have it the other way
    tokens.reverse();
    (name, Type(tokens), bitfield)
}

fn recursive_parse(
    l: &mut Lexer,
    tokens: &mut Vec<TyToken>,
    name: &mut Option<UniqueStr>,
    bitfield: &mut Option<u8>,
    int: &Interner,
) {
    // in declarations where no parentheses are used this never allocates (in fact I don't think vulkan uses any)
    let mut later = Vec::new();
    while let Some(next) = l.next() {
        match next {
            Token::Ptr => tokens.push(TyToken::Ptr),
            Token::Const => tokens.push(TyToken::Const),
            Token::LBracket => {
                let size = match l.next().unwrap() {
                    Token::Alphanumeric(s) => {
                        let s = s.intern(int);
                        l.expect(Token::RBracket);
                        Some(s)
                    }
                    Token::RBracket => None,
                    _ => unreachable!(),
                };
                tokens.push(TyToken::Array(size))
            }
            Token::LParen => {
                recursive_parse(l, &mut later, name, bitfield, int);
                l.expect(Token::RParen);

                if bitfield.is_some() {
                    break;
                }
            }
            Token::Alphanumeric(s) => {
                // any alphanumeric in this place must be a name
                assert!(name.is_none());
                *name = Some(s.intern(int));
            }
            Token::DoubleColon => {
                if let Some(Token::Alphanumeric(bits)) = l.next() {
                    assert!(bitfield.is_none());
                    *bitfield = Some(bits.parse().unwrap());
                    break;
                } else {
                    unreachable!()
                }
            }
            Token::RBracket | Token::RParen => unreachable!(),
        }

        if let Some(Token::RParen) = l.peek() {
            break;
        }
    }

    tokens.append(&mut later)
}

#[test]
fn test_lex_type() {
    let data: &[(&str, &[Token])] = &[
        (
            "const char*",
            &[Token::Const, Token::Alphanumeric("char"), Token::Ptr],
        ),
        (
            "ůůAaůů *   I ඞ",
            &[
                // only recognizes ascii characters
                Token::Alphanumeric("Aa"),
                Token::Ptr,
                Token::Alphanumeric("I"),
            ],
        ),
        (
            "name1 2 ** [size] \n next ",
            &[
                Token::Alphanumeric("name1"),
                Token::Alphanumeric("2"),
                Token::Ptr,
                Token::Ptr,
                Token::LBracket,
                Token::Alphanumeric("size"),
                Token::RBracket,
                Token::Alphanumeric("next"),
            ],
        ),
    ];

    for (str, expect) in data {
        let tokens = CLexer::new(str).collect::<Vec<_>>();
        assert_eq!(tokens.as_slice(), *expect);
    }
}
#[test]
fn test_parse_type() {
    let int = Interner::new();
    // verified by https://cdecl.org/
    let data: &[(&str, Option<&str>, &[TyToken])] = &[
        (
            "const char*",
            None,
            &[
                TyToken::Ptr,
                TyToken::Const,
                TyToken::BaseType("char".intern(&int)),
            ],
        ),
        (
            "char (*(*name)[1][2])[3]",
            Some("name"),
            &[
                TyToken::Ptr,
                TyToken::Array(Some("1".intern(&int))),
                TyToken::Array(Some("2".intern(&int))),
                TyToken::Ptr,
                TyToken::Array(Some("3".intern(&int))),
                TyToken::BaseType("char".intern(&int)),
            ],
        ),
    ];

    for &(str, name, tokens) in data {
        let expect = (name, tokens);

        // borrowchk fun!
        let got = parse_type_decl(str, &int);
        let mut tmp = None;
        let got = (
            got.0.map(|s| {
                tmp = Some(s);
                tmp.as_ref().unwrap().resolve()
            }),
            got.1 .0.as_slice(),
        );

        assert_eq!(got, expect);
    }
}

#[test]
fn test_type_display() {
    let int = Interner::new();
    let ty = Type(vec![
        TyToken::Ptr,
        TyToken::Array(None),
        TyToken::Array(Some("1".intern(&int))),
        TyToken::Ptr,
        TyToken::Const,
        TyToken::BaseType("char".intern(&int)),
    ]);

    let display = format!("{}", ty);
    let debug = format!("{:?}", ty);

    assert_eq!(display, "*mut [[*const char; 1]]");
    assert_eq!(debug, "\"*mut [[*const char; 1]]\"");
}
