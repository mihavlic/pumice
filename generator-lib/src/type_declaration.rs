use std::{
    borrow::{Borrow, Cow},
    fmt::{Debug, Display, Write},
    iter::Peekable,
    ops::{Deref, DerefMut},
};

use smallvec::SmallVec;

use crate::{interner::UniqueStr, Intern, Interner};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NumberLiteral {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Usize(usize),
    Isize(isize),
}

impl NumberLiteral {
    #[track_caller]
    pub fn get_integer(&self) -> i64 {
        match *self {
            // unsigned
            NumberLiteral::U32(i) => TryInto::<i64>::try_into(i).unwrap(),
            NumberLiteral::U64(i) => TryInto::<i64>::try_into(i).unwrap(),
            NumberLiteral::Usize(i) => TryInto::<i64>::try_into(i).unwrap(),
            // signed
            NumberLiteral::I32(i) => TryInto::<i64>::try_into(i).unwrap(),
            NumberLiteral::I64(i) => i,
            NumberLiteral::Isize(i) => TryInto::<i64>::try_into(i).unwrap(),
            // float, bleh
            NumberLiteral::F32(_) | NumberLiteral::F64(_) => {
                unreachable!("Arrays cannot be sized by a floating point value.")
            }
        }
    }
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberLiteral::U32(a) => Display::fmt(a, f),
            NumberLiteral::I32(a) => Display::fmt(a, f),
            NumberLiteral::U64(a) => Display::fmt(a, f),
            NumberLiteral::I64(a) => Display::fmt(a, f),
            NumberLiteral::F32(a) => Display::fmt(a, f),
            NumberLiteral::F64(a) => Display::fmt(a, f),
            NumberLiteral::Usize(a) => Display::fmt(a, f),
            NumberLiteral::Isize(a) => Display::fmt(a, f),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Number(NumberLiteral),
    Ident(&'a str),
    Struct,
    Const,
    LBracket,
    RBracket,
    LParen,
    RParen,
    LBrace,
    RBrace,
    DoubleColon,
    // operators
    Tilde,
    Exclamation,
    Percent,
    Caret,
    Ampersand,
    Star,
    Slash,
    Plus,
    Minus,
    // field acess
    Dot,
    Arrow,
    // misc
    Unknown(char),
    End,
}

// "inspired" by https://github.com/gfx-rs/naga/blob/48e79388b506535d668df4f6c7be4e681812ab81/src/front/wgsl/lexer.rs#L8
pub fn next_token(mut str: &str) -> (Token<'_>, &str) {
    let mut chars = str.chars();
    loop {
        str = chars.as_str();
        let cur = match chars.next() {
            Some(c) => c,
            None => return (Token::End, ""),
        };

        macro_rules! two_char_token {
            ($($char:pat => $token:expr),+ ; $single:expr) => {
                match chars.clone().next() {
                    $(
                        Some($char) => {
                            chars.next();
                            $token
                        },
                    )+
                        _  => $single
                }
            };
        }

        let tok = match cur {
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' => Token::DoubleColon,
            '~' => Token::Tilde,
            '!' => Token::Exclamation,
            '%' => Token::Percent,
            '^' => Token::Caret,
            '&' => Token::Ampersand,
            '*' => Token::Star,
            '/' => Token::Slash,
            '+' => Token::Plus,
            '-' => two_char_token!(
                '>' => Token::Arrow
                ; Token::Minus
            ),
            '.' => two_char_token!(
                '0'..='9' => {
                    let (number, next) = consume_number(str);
                    return (Token::Number(number), next);
                }
                ; Token::Dot
            ),
            '0'..='9' => {
                let (number, next) = consume_number(str);
                return (Token::Number(number), next);
            }
            _ if is_blankspace(cur) => continue,
            _ if is_identifier(cur) => {
                let (ident, rest) = consume_any(str, is_identifier);
                let tok = match ident {
                    "const" => Token::Const,
                    "struct" => Token::Struct,
                    _ => Token::Ident(ident),
                };
                return (tok, rest);
            }
            _ => Token::Unknown(cur),
        };

        return (tok, chars.as_str());
    }
}

fn consume_any(input: &str, what: impl Fn(char) -> bool) -> (&str, &str) {
    let pos = input.find(|c| !what(c)).unwrap_or(input.len());
    input.split_at(pos)
}

fn is_blankspace(c: char) -> bool {
    c.is_whitespace()
}

fn is_identifier(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn consume_number(str: &str) -> (NumberLiteral, &str) {
    // https://en.cppreference.com/w/cpp/language/integer_literal
    // https://stackoverflow.com/a/5026592
    // 1.01
    // .01
    // 1
    // 1.0F
    // 1ULL

    #[derive(PartialEq, Eq)]
    enum State {
        Start,
        Hexadecimal,
        Decimal,
        Fractional,
        Suffix,
    }

    let mut state = State::Start;
    let mut chars = str.char_indices();

    let mut is_hexadecimal = false;
    let mut is_fractional = false;
    let mut suffix_start = None;
    let mut string_end = None;
    while let Some((i, c)) = chars.next() {
        match state {
            State::Start => match c {
                '0' if matches!(chars.clone().next().map(|(_, c)| c), Some('x' | 'X')) => {
                    chars.next();
                    is_hexadecimal = true;
                    state = State::Hexadecimal;
                }
                '0'..='9' => state = State::Decimal,
                '.' => {
                    is_fractional = true;
                    state = State::Fractional;
                }
                _ => panic!("Unexpected character '{}' : \"{}\"", c, &str[..=i]),
            },
            State::Decimal => match c {
                '.' => {
                    is_fractional = true;
                    state = State::Fractional;
                }
                '0'..='9' => {}
                'U' | 'u' | 'L' | 'l' | 'Z' | 'z' => {
                    suffix_start = Some(i);
                    state = State::Suffix;
                }
                _ => {
                    string_end = Some(i);
                    break;
                }
            },
            State::Hexadecimal => match c {
                '0'..='9' | 'a'..='f' => {}
                _ if c.is_whitespace() => {
                    string_end = Some(i);
                }
                _ => {
                    string_end = Some(i);
                    break;
                }
            },
            State::Fractional => match c {
                '0'..='9' => {}
                'U' | 'u' | 'L' | 'l' | 'Z' | 'z' => {
                    suffix_start = Some(i);
                    state = State::Suffix;
                }
                _ => {
                    string_end = Some(i);
                    break;
                }
            },
            State::Suffix => match c {
                'U' | 'u' | 'L' | 'l' | 'Z' | 'z' => {}
                _ => {
                    string_end = Some(i);
                    break;
                }
            },
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Longness {
        None,
        Long,
        LongLong,
    }

    let mut longness = Longness::None;
    let mut unsigned = false;
    let mut float = false;
    let mut size = false;

    if let Some(suffix) = suffix_start {
        let suffix = &str[suffix..string_end.unwrap_or(str.len())];

        if suffix.contains(['U', 'u']) {
            unsigned = true;
        }
        if suffix.contains(['F', 'f']) {
            float = true;
        }
        if suffix.contains(['Z', 'z']) {
            size = true;
        }
        if suffix.contains("LL") || suffix.contains("ll") {
            longness = Longness::LongLong;
        } else if suffix.contains(['L', 'l']) {
            longness = Longness::Long;
        }
    }

    let end = suffix_start.or(string_end).unwrap_or(str.len());
    let number = if is_hexadecimal {
        &str[2..end]
    } else {
        &str[..end]
    };

    macro_rules! int {
        ($int:ident) => {
            if is_hexadecimal {
                $int::from_str_radix(number, 16).unwrap()
            } else {
                $int::from_str_radix(number, 10).unwrap()
            }
        };
    }

    let number = match (is_hexadecimal, longness, unsigned, is_fractional, float, size) {
        (_, Longness::None, true, false, false, false) => NumberLiteral::U32(int!(u32)),
        (_, Longness::None, false, false, false, false) => NumberLiteral::I32(int!(i32)),
        (_, Longness::Long | Longness::LongLong, true, false, false, false) => NumberLiteral::U64(int!(u64)),
        (_, Longness::Long | Longness::LongLong, false, false, false, false) => NumberLiteral::I64(int!(i64)),
        (false, Longness::None, false, true, true, false) => NumberLiteral::F32(number.parse().unwrap()),
        (false, Longness::None, false, true, false, false) => NumberLiteral::F64(number.parse().unwrap()),
        (_, Longness::None, true, false, false, true) => NumberLiteral::Usize(int!(usize)),
        (_, Longness::None, false, false, false, true) => NumberLiteral::Isize(int!(isize)),
        _ => panic!(
            "\
Error parsing '{number}'.
The following combination is invalid:
    (hexadecimal: {is_hexadecimal}, longness: {:?}, unsigned: {unsigned}, fractional: {is_fractional}, float: {float}, size: {size})",
    longness
        )
    };
    (number, &str[end..])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyToken {
    BaseType(UniqueStr),
    Const,
    Ptr,
    Ref,
    Array(Option<UniqueStr>),
}

// in release mode the size of TyToken is 16 bytes, the maximum size
// which allows the SmallVec to be the same size as a Vec
#[derive(PartialEq, Eq, Clone)]
pub struct Type(pub SmallVec<[TyToken; 1]>);

impl Deref for Type {
    type Target = TypeRef;

    #[inline]
    fn deref(&self) -> &Self::Target {
        TypeRef::from_slice(&self.0)
    }
}

impl DerefMut for Type {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        TypeRef::from_slice_mut(&mut self.0)
    }
}

impl Type {
    pub fn from_only_basetype(basetype: UniqueStr) -> Self {
        Self(smallvec::smallvec![TyToken::BaseType(basetype)])
    }
    // Since C is such a lovely language, some types have different semantics when they are used as a function argument.
    // For example, consider:
    //   void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4]) {}
    // the `const float [4]` actually means (in rust) `*const [float; 4]`
    // this means that types in function arguments must be specially handled
    pub fn make_c_function_arg(&mut self) {
        if self
            .as_slice()
            .iter()
            .find(|t| matches!(t, TyToken::Array(_)))
            .is_none()
        {
            return;
        }

        let mut out = SmallVec::new();
        let mut tokens = self.as_slice().iter().peekable();

        while let Some(&token) = tokens.next() {
            match token {
                TyToken::Array(_) => {
                    out.push(TyToken::Ptr);
                    // if we have a situation like
                    //   `const char[4]`
                    //   Arr(4), Const, Ident(Char)
                    // we would get
                    //   Ptr, Arr(4), Const, Ident(Char)
                    // yet we want
                    //   Ptr, Const, Arr(4), Ident(Char)
                    // which is obviously invalid, so if we see a Const when
                    // emitting a pointer, we eat it from the Arr and re-emit it
                    // after the pointer, after which the actual array is emitted
                    if let Some(TyToken::Const) = tokens.peek() {
                        out.push(TyToken::Const);
                        tokens.next();
                    }
                }
                _ => {}
            }
            out.push(token);
        }

        *self = Type(out);
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct TypeRef([TyToken]);

impl TypeRef {
    pub fn from_slice(slice: &[TyToken]) -> &TypeRef {
        // sound because the type is transparent
        unsafe { std::mem::transmute::<_, &TypeRef>(slice) }
    }
    pub fn as_slice(&self) -> &[TyToken] {
        &self.0
    }
    pub fn from_slice_mut(slice: &mut [TyToken]) -> &mut TypeRef {
        // sound because the type is transparent
        unsafe { std::mem::transmute::<_, &mut TypeRef>(slice) }
    }
    pub fn try_only_basetype(&self) -> Option<UniqueStr> {
        // FIXME if it is valid to have only a const basetype then this function does not work
        if self.0.len() == 1 {
            if let &TyToken::BaseType(s) = &self.0[0] {
                return Some(s);
            }
        }

        None
    }
    pub fn try_not_only_basetype(&self) -> Option<UniqueStr> {
        if self.0.len() != 1 {
            return Some(self.get_basetype());
        }

        None
    }
    pub fn get_basetype(&self) -> UniqueStr {
        if let TyToken::BaseType(s) = self.0.last().unwrap() {
            *s
        } else {
            unreachable!()
        }
    }
    pub fn get_basetype_mut(&mut self) -> &mut UniqueStr {
        if let TyToken::BaseType(s) = self.0.last_mut().unwrap() {
            s
        } else {
            unreachable!()
        }
    }
    pub fn descend(&self) -> &TypeRef {
        match &self.0[0] {
            TyToken::Ptr | TyToken::Ref => {
                if let Some(TyToken::Const) = self.0.get(1) {
                    return TypeRef::from_slice(&self.0[2..]);
                }
            }
            TyToken::Const => {
                assert!(matches!(self.0.get(1), Some(TyToken::BaseType(_))));
                // all is well, pass and strip the const
            }
            TyToken::BaseType(_) => {
                // cannot descend further
                return &self;
            }
            _ => {}
        }
        TypeRef::from_slice(&self.0[1..])
    }
    pub fn first(&self) -> &TyToken {
        &self.0[0]
    }
    pub fn first_mut(&mut self) -> &mut TyToken {
        &mut self.0[0]
    }
    pub fn ptr_to_ref(&self) -> Cow<'_, TypeRef> {
        match self.first() {
            &TyToken::Ptr => {
                let mut owned = self.to_owned();
                *owned.first_mut() = TyToken::Ref;
                Cow::Owned(owned)
            }
            _ => Cow::Borrowed(&self),
        }
    }
    pub fn strip_indirection(&self) -> &TypeRef {
        match self.first() {
            TyToken::Ptr | TyToken::Ref => self.descend(),
            _ => &self,
        }
    }

    // Since C is such a lovely language, some types have different semantics when they are used as a function argument.
    // For example, consider:
    //   void vkCmdSetBlendConstants(VkCommandBuffer commandBuffer, const float blendConstants[4]) {}
    // the `const float [4]` actually means (in rust) `*const [float; 4]`
    // this means that types in function arguments must be specially handled
    pub fn to_c_function_arg(&self) -> Cow<'_, TypeRef> {
        if self
            .as_slice()
            .iter()
            .find(|t| matches!(t, TyToken::Array(_)))
            .is_none()
        {
            return Cow::Borrowed(self);
        }

        let mut out = SmallVec::new();
        let mut tokens = self.as_slice().iter().peekable();

        while let Some(&token) = tokens.next() {
            match token {
                TyToken::Array(_) => {
                    out.push(TyToken::Ptr);
                    // if we have a situation like
                    //   `const char[4]`
                    //   Arr(4), Const, Ident(Char)
                    // we would get
                    //   Ptr, Arr(4), Const, Ident(Char)
                    // yet we want
                    //   Ptr, Const, Arr(4), Ident(Char)
                    // which is obviously invalid, so if we see a Const when
                    // emitting a pointer, we eat it from the Arr and re-emit it
                    // after the pointer, after which the actual array is emitted
                    if let Some(TyToken::Const) = tokens.peek() {
                        out.push(TyToken::Const);
                        tokens.next();
                    }
                }
                _ => {}
            }
            out.push(token);
        }

        Cow::Owned(Type(out))
    }
}

impl ToOwned for TypeRef {
    type Owned = Type;

    fn to_owned(&self) -> Self::Owned {
        Type(SmallVec::from_slice(&self.0))
    }
}

impl Borrow<TypeRef> for Type {
    fn borrow(&self) -> &TypeRef {
        &self
    }
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_type_tokens_simple(&self.0, f)
    }
}

impl Debug for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        fmt_type_tokens_simple(&self.0, f)?;
        f.write_char('"')
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <TypeRef as Display>::fmt(self.deref(), f)
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <TypeRef as Debug>::fmt(self.deref(), f)
    }
}

fn fmt_type_tokens_simple(tokens: &[TyToken], f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let resolve = |s: UniqueStr, f: &mut std::fmt::Formatter| f.write_str(s.resolve());
    fmt_type_tokens_impl(tokens, &resolve, &resolve, f)
}

pub fn fmt_type_tokens_impl<
    T: std::fmt::Write,
    F1: Fn(UniqueStr, &mut T) -> std::fmt::Result,
    F2: Fn(UniqueStr, &mut T) -> std::fmt::Result,
>(
    tokens: &[TyToken],
    on_ident: &F1,
    on_arr: &F2,
    f: &mut T,
) -> std::fmt::Result {
    let mut tokens_iter = tokens.iter().enumerate();
    while let Some((i, token)) = tokens_iter.next() {
        match token {
            TyToken::BaseType(s) => {
                on_ident(*s, f)?;
            }
            TyToken::Ref => {
                f.write_char('&')?;

                match tokens.get(i + 1) {
                    Some(TyToken::Const) => {
                        tokens_iter.next();
                    }
                    _ => {
                        f.write_str("mut ")?;
                    }
                }

                continue;
            }
            TyToken::Ptr => {
                f.write_char('*')?;

                if !matches!(tokens.get(i + 1), Some(&TyToken::Const)) {
                    f.write_str("mut ")?;
                }

                continue;
            }
            TyToken::Array(s) => {
                f.write_char('[')?;
                fmt_type_tokens_impl(&tokens[i + 1..], on_ident, on_arr, f)?;
                if let Some(size) = s {
                    f.write_str("; ")?;
                    on_arr(*size, f)?;
                }
                f.write_char(']')?;
                return Ok(());
            }
            TyToken::Const => {
                f.write_str("const")?;
            }
        }

        if i != tokens.len() - 1 {
            f.write_char(' ')?;
        }
    }
    Ok(())
}

pub struct TokenIter<'a>(&'a str);

impl<'a> TokenIter<'a> {
    pub fn new(str: &'a str) -> Self {
        Self(str)
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, next) = next_token(&self.0);
        self.0 = next;
        (token != Token::End).then(|| token)
    }
}

pub struct Lexer<'a, T: Iterator<Item = Token<'a>> + 'a>(Peekable<T>);

impl<'a, T: Iterator<Item = Token<'a>> + 'a> Lexer<'a, T> {
    pub fn new(iter: T) -> Self {
        let iter = iter.peekable();
        Self(iter)
    }
    pub fn next(&mut self) -> Option<Token> {
        self.0.next()
    }
    pub fn peek(&mut self) -> Option<Token> {
        self.0.peek().cloned()
    }
    pub fn expect(&mut self, tok: Token) {
        let next = self.next();
        assert!(Some(tok) == next, "Expected '{:?}', got '{:?}'", tok, next);
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

struct NoStructFilter<'a>(TokenIter<'a>);
impl<'a> Iterator for NoStructFilter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(Token::Struct) => continue,
                other => return other,
            }
        }
    }
}

type TypeLexer<'a> = Lexer<'a, NoStructFilter<'a>>;

// Bloody hell, C type decls are such a mess!
// Resources used:
//   https://eli.thegreenplace.net/2008/07/18/reading-c-type-declarations/
//   http://www.unixwiz.net/techtips/reading-cdecl.html
//   https://cdecl.org/
// TODO support function pointers
pub fn parse_type_decl(str: &str, int: &Interner) -> (Option<UniqueStr>, Type, Option<u8>) {
    let mut l = Lexer::new(NoStructFilter(TokenIter::new(str)));
    let parens = l.consume_all(Token::LParen);

    let mut tokens = SmallVec::new();

    match l.next().unwrap() {
        Token::Ident(s) => {
            tokens.push(TyToken::BaseType(s.intern(int)));
            if let Some(Token::Const) = l.peek() {
                tokens.push(TyToken::Const);
                l.next();
            }
        }
        Token::Const => {
            if let Some(Token::Ident(s)) = l.next() {
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
    l: &mut TypeLexer,
    tokens: &mut SmallVec<[TyToken; 1]>,
    name: &mut Option<UniqueStr>,
    bitfield: &mut Option<u8>,
    int: &Interner,
) {
    // in declarations where no parentheses are used this never allocates (in fact I don't think vulkan uses any)
    let mut later = SmallVec::new();
    while let Some(next) = l.next() {
        match next {
            Token::Star => tokens.push(TyToken::Ptr),
            Token::Const => tokens.push(TyToken::Const),
            Token::LBracket => {
                let size = match l.next().unwrap() {
                    Token::Ident(s) => {
                        let s = s.intern(int);
                        l.expect(Token::RBracket);
                        Some(s)
                    }
                    Token::Number(num) => {
                        l.expect(Token::RBracket);
                        let len: usize = num.get_integer().try_into().unwrap();
                        let mut buf: SmallVec<[u8; 16]> = SmallVec::new();
                        use std::io::Write;
                        write!(buf, "{}", len).unwrap();
                        Some(unsafe { std::str::from_utf8_unchecked(buf.as_slice()).intern(int) })
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
            Token::Ident(s) => {
                // any alphanumeric in this place must be a name
                assert!(name.is_none());
                *name = Some(s.intern(int));
            }
            Token::DoubleColon => match l.next() {
                // technically preprocessor defines could be used here, however I quite don't care
                Some(Token::Ident(_)) => {
                    panic!("Currently cannot use symbols for bitfields.")
                }
                Some(Token::Number(num)) => {
                    assert!(bitfield.is_none());
                    *bitfield = Some(num.get_integer().try_into().unwrap());
                    break;
                }
                _ => unreachable!(),
            },
            _ => unreachable!("Unexpected token '{:?}'", next),
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
            "null-terminated",
            &[
                Token::Ident("null"),
                Token::Minus,
                Token::Ident("terminated"),
            ],
        ),
        (
            "const char*",
            &[Token::Const, Token::Ident("char"), Token::Star],
        ),
        (
            "ůůAaůů *   I ඞ",
            &[
                // only recognizes ascii characters
                Token::Ident("Aa"),
                Token::Star,
                Token::Ident("I"),
            ],
        ),
        (
            "name1 2 ** [size] \n next ",
            &[
                Token::Ident("name1"),
                Token::Ident("2"),
                Token::Star,
                Token::Star,
                Token::LBracket,
                Token::Ident("size"),
                Token::RBracket,
                Token::Ident("next"),
            ],
        ),
    ];

    for (str, expect) in data {
        let tokens = TokenIter::new(str).collect::<Vec<_>>();
        assert_eq!(tokens.as_slice(), *expect);
    }
}

#[test]
fn test_parse_type() {
    let int = Interner::new();
    // verified by https://cdecl.org/
    let data: &[(&str, (Option<&str>, &[TyToken], Option<u8>))] = &[
        (
            "const char*",
            (
                None,
                &[
                    TyToken::Ptr,
                    TyToken::Const,
                    TyToken::BaseType("char".intern(&int)),
                ],
                None,
            ),
        ),
        (
            "char (*(*name)[1][2])[3]",
            (
                Some("name"),
                &[
                    TyToken::Ptr,
                    TyToken::Array(Some("1".intern(&int))),
                    TyToken::Array(Some("2".intern(&int))),
                    TyToken::Ptr,
                    TyToken::Array(Some("3".intern(&int))),
                    TyToken::BaseType("char".intern(&int)),
                ],
                None,
            ),
        ),
        (
            "const float blendConstants [4]",
            (
                Some("blendConstants"),
                &[
                    TyToken::Array(Some("4".intern(&int))),
                    TyToken::Const,
                    TyToken::BaseType("float".intern(&int)),
                ],
                None,
            ),
        ),
    ];

    for &(str, expect) in data {
        let got = parse_type_decl(str, &int);

        let got_ = (
            got.0.as_ref().map(|s| s.resolve()),
            got.1 .0.as_slice(),
            got.2,
        );

        assert_eq!(got_, expect);
    }
}

#[test]
fn test_type_display() {
    use smallvec::smallvec;

    let int = Interner::new();
    let ty = Type(smallvec![
        TyToken::Ref,
        TyToken::Ref,
        TyToken::Const,
        TyToken::Ptr,
        TyToken::Array(None),
        TyToken::Array(Some("1".intern(&int))),
        TyToken::Ptr,
        TyToken::Const,
        TyToken::BaseType("char".intern(&int)),
    ]);

    let display = format!("{}", ty);
    let debug = format!("{:?}", ty);

    assert_eq!(display, "&mut &*mut [[*const char; 1]]");
    assert_eq!(debug, "\"&mut &*mut [[*const char; 1]]\"");
}
