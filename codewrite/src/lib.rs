pub mod formatters;
pub mod write;

use std::{
    cell::Cell,
    fmt::{Display, Write},
    marker::PhantomData,
};

pub trait WriteLast: Write {
    fn last_char(&self) -> Option<char>;
    fn separate_whitespace(&mut self) {
        if let Some(last) = self.last_char() {
            if !last.is_ascii_whitespace() {
                self.write_char(' ').unwrap();
            }
        }
    }
}

pub struct WriterLastWriter<W: Write>(W, Option<char>);

impl<W: Write> WriteLast for WriterLastWriter<W> {
    fn last_char(&self) -> Option<char> {
        self.1
    }
}

impl WriteLast for String {
    fn last_char(&self) -> Option<char> {
        self.chars().rev().next()
    }
}

impl<W: Write> Write for WriterLastWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if !s.is_empty() {
            self.1 = Some(s.chars().rev().next().unwrap());
            self.0.write_str(s)
        } else {
            Ok(())
        }
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.1 = Some(c);
        self.0.write_char(c)
    }
}

pub trait CFmt<W: WriteLast, C> {
    fn cfmt(&self, w: &mut W, ctx: &mut C);
}

impl<W: WriteLast, C, F: Fn(&mut W, &mut C)> CFmt<W, C> for F {
    fn cfmt(&self, w: &mut W, ctx: &mut C) {
        (self)(w, ctx)
    }
}

impl<W: WriteLast, C, F: FnOnce(&mut W, &mut C)> CFmt<W, C> for SingleCell<F> {
    fn cfmt(&self, w: &mut W, ctx: &mut C) {
        (self.get())(w, ctx)
    }
}

#[derive(Clone)]
pub struct Cond<W: WriteLast, C, T: CFmt<W, C>>(pub bool, pub T, PhantomData<*const (W, C)>);

impl<W: WriteLast, C, T: CFmt<W, C>> Cond<W, C, T> {
    pub fn new(cond: bool, val: T) -> Self {
        Self(cond, val, PhantomData)
    }
}

impl<W: WriteLast, C, T: CFmt<W, C>> CFmt<W, C> for Cond<W, C, T> {
    fn cfmt(&self, w: &mut W, ctx: &mut C) {
        let Cond(cond, val, _) = self;
        if *cond {
            val.cfmt(w, ctx)
        }
    }
}

struct SingleCell<T>(Cell<Option<T>>);

impl<T: Copy> Clone for SingleCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SingleCell<T> {
    fn new(val: T) -> Self {
        Self(Cell::new(Some(val)))
    }
    fn get(&self) -> T {
        self.0.take().unwrap()
    }
}

pub struct Iter<W: WriteLast, C, T, I: IntoIterator<Item = T>, F: FnMut(&mut W, &mut C, T)>(
    SingleCell<(I, F)>,
    PhantomData<*const (W, C)>,
);

impl<W: WriteLast, C, T, I: IntoIterator<Item = T>, F: FnMut(&mut W, &mut C, T)> CFmt<W, C>
    for Iter<W, C, T, I, F>
{
    fn cfmt(&self, w: &mut W, ctx: &mut C) {
        let (iter, mut fun) = self.0.get();
        for it in iter {
            fun(w, ctx, it);
        }
    }
}

pub struct Concat<'a>(pub [&'a dyn Display]);

impl<'a> Display for Concat<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &it in &self.0 {
            write!(f, "{it}")?;
        }
        Ok(())
    }
}

pub struct Separated<
    'a,
    W: WriteLast,
    C,
    T: IntoIterator,
    F: Fn(&mut W, &mut C, <T as IntoIterator>::Item),
> {
    iter_fun: SingleCell<(T, F)>,
    sep: &'a str,
    // whether to add a separator after the last element
    sep_last: bool,
    spooky: std::marker::PhantomData<(W, C)>,
}

impl<'a, T: IntoIterator, W: WriteLast, C, F: Fn(&mut W, &mut C, <T as IntoIterator>::Item)> Clone
    for Separated<'a, W, C, T, F>
where
    T: Copy,
    F: Copy,
{
    fn clone(&self) -> Self {
        Self {
            iter_fun: self.iter_fun.clone(),
            sep: self.sep.clone(),
            sep_last: self.sep_last.clone(),
            spooky: std::marker::PhantomData,
        }
    }
}

impl<'a, T: IntoIterator, W: WriteLast, C, F: Fn(&mut W, &mut C, <T as IntoIterator>::Item)>
    Separated<'a, W, C, T, F>
{
    pub fn new(iter: T, fun: F, separator: &'a str, separator_last: bool) -> Self {
        Self {
            iter_fun: SingleCell::new((iter, fun)),
            sep: separator,
            sep_last: separator_last,
            spooky: std::marker::PhantomData,
        }
    }
    pub fn args(iter: T, fun: F) -> Self {
        Self::new(iter, fun, ",", false)
    }
    pub fn statements(iter: T, fun: F) -> Self {
        Self::new(iter, fun, ";", true)
    }
}

impl<'a, W: WriteLast, T: IntoIterator>
    Separated<'a, W, (), T, fn(&mut W, &mut (), <T as IntoIterator>::Item)>
where
    <T as IntoIterator>::Item: CFmt<W, ()>,
{
    pub fn display(iter: T, separator: &'a str) -> Self {
        Self::new(
            iter,
            |w: &mut W, ctx: &mut (), a: <T as IntoIterator>::Item| a.cfmt(w, ctx),
            separator,
            false,
        )
    }
}

impl<'a, W: WriteLast, C, T: IntoIterator, F: Fn(&mut W, &mut C, <T as IntoIterator>::Item)>
    CFmt<W, C> for Separated<'a, W, C, T, F>
{
    fn cfmt(&self, w: &mut W, ctx: &mut C) {
        let (iter, fun) = self.iter_fun.get();
        let mut iter = iter.into_iter().peekable();
        while let Some(next) = iter.next() {
            fun(w, ctx, next);

            if iter.peek().is_some() || self.sep_last {
                w.write_str(self.sep).unwrap();
            }
        }
    }
}

macro_rules! impl_display {
    ($($(#[$($meta:tt)+])? $T:ty $(where $for:tt)?),+) => {
        $(
            $(#[$($meta)+])?
            impl<W: WriteLast, C> CFmt<W, C> for $T $($for)? {
                fn cfmt(&self, w: &mut W, _: &mut C) {
                    write!(w, "{}", self).unwrap();
                }
            }
        )+
    };
}

impl_display!(
    std::convert::Infallible,
    std::env::VarError,
    std::io::ErrorKind,
    std::net::IpAddr,
    std::net::SocketAddr,
    std::sync::mpsc::RecvTimeoutError,
    std::sync::mpsc::TryRecvError,
    bool,
    char,
    f32,
    f64,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    str,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    // core::ffi::FromBytesUntilNulError,
    // std::alloc::AllocError,
    std::alloc::LayoutError,
    std::array::TryFromSliceError,
    std::backtrace::Backtrace,
    std::cell::BorrowError,
    std::cell::BorrowMutError,
    std::char::CharTryFromError,
    std::char::DecodeUtf16Error,
    std::char::ParseCharError,
    std::char::ToLowercase,
    std::char::ToUppercase,
    std::char::TryFromCharError,
    std::collections::TryReserveError,
    std::env::JoinPathsError,
    std::ffi::FromBytesWithNulError,
    std::ffi::FromVecWithNulError,
    std::ffi::IntoStringError,
    std::ffi::NulError,
    std::io::WriterPanicked,
    std::net::AddrParseError,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize,
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize,
    std::num::ParseFloatError,
    std::num::ParseIntError,
    std::num::TryFromIntError,
    #[cfg(target_os = "windows")]
    InvalidHandleError,
    #[cfg(target_os = "windows")]
    NullHandleError,
    std::path::StripPrefixError,
    std::process::ExitStatus,
    // std::process::ExitStatusError,
    std::str::ParseBoolError,
    std::str::Utf8Error,
    std::string::FromUtf8Error,
    std::string::FromUtf16Error,
    String,
    std::sync::mpsc::RecvError,
    std::thread::AccessError,
    std::time::SystemTimeError,
    std::time::TryFromFloatSecsError,
    #[cfg(feature = "proc_macro")]
    LexError,
    #[cfg(feature = "proc_macro")]
    ExpandError,
    #[cfg(feature = "proc_macro")]
    TokenStream,
    #[cfg(feature = "proc_macro")]
    TokenTree,
    #[cfg(feature = "proc_macro")]
    Group,
    #[cfg(feature = "proc_macro")]
    Punct,
    #[cfg(feature = "proc_macro")]
    Ident,
    #[cfg(feature = "proc_macro")]
    Literal
);

pub struct Format2Format<W: std::io::Write>(pub W);

impl<W: std::io::Write> std::fmt::Write for Format2Format<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}
