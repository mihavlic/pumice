use std::{
    cell::Cell,
    fmt::{Arguments, Display, Formatter, Write},
};

use generator_lib::type_declaration::TypeDecl;
use lasso::Spur;

use crate::Registry;

pub struct Separated<
    T: Iterator,
    F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result,
> {
    iter: Cell<Option<T>>,
    fun: F,
    // the string before writing the content of the iterator
    pad: &'static str,
    // the string after writing the content of the iterator
    end: &'static str,
    // whether to write the end string even for the last element, this is useful for separated but not actually delimited elements
    // such as function arguments
    end_last: bool,
    // whether to separate elements with a newline along with the 'end' string padding
    // the difference here is that we want to emit newlines for every but the last element
    // but if end_last is set we want to write the end string for every single one
    newline: bool,
}

impl<T: Iterator, F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result>
    Separated<T, F>
{
    pub fn new(
        iter: T,
        fun: F,
        pad: &'static str,
        end: &'static str,
        newline: bool,
        end_last: bool,
    ) -> Self {
        Self {
            iter: Cell::new(Some(iter)),
            fun,
            pad,
            end,
            end_last,
            newline,
        }
    }
    pub fn args(iter: T, fun: F) -> Self {
        Self::new(iter, fun, ", ", "", false, false)
    }
    pub fn members(iter: T, fun: F) -> Self {
        Self::new(iter, fun, "    ", ",", true, false)
    }
    pub fn statements(iter: T, fun: F) -> Self {
        Self::new(iter, fun, "    ", ";", true, true)
    }
}

impl<T: Iterator, F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result> Display
    for Separated<T, F>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.replace(None).unwrap().peekable(); // cursed
        let mut first = true;
        while let Some(next) = iter.next() {
            let last = iter.peek().is_none();
            if !first {
                f.write_str(self.pad)?;
            }

            first = false;
            (self.fun)(next, f)?;

            if !last || self.end_last {
                f.write_str(self.end)?;
            }
            if !last && self.newline {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

pub struct WithRegistry<'a, T: ?Sized>(&'a Registry, T);

pub trait RegistryWrap {
    fn reg(self, reg: &Registry) -> WithRegistry<Self>;
}

impl<T> RegistryWrap for T {
    fn reg(self, reg: &Registry) -> WithRegistry<Self> {
        WithRegistry(reg, self)
    }
}

pub trait RegistryDisplay {
    fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<'a, T: RegistryDisplay> Display for WithRegistry<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RegistryDisplay::format(&self.1, self.0, f)
    }
}

#[macro_export]
macro_rules! code {
    ($buf:expr, $($code:literal)+ @ $($tail:tt)*) => {
        write!(
            &mut $buf,
            concat!($($code, "\n"),+),
            $(
                $tail
            )*
        )
    }
}

impl RegistryDisplay for Arguments<'_> {
    fn format(&self, _: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(*self)
    }
}

impl RegistryDisplay for &str {
    fn format(&self, _: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

impl RegistryDisplay for String {
    fn format(&self, _: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RegistryDisplay for u32 {
    fn format(&self, _: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl RegistryDisplay for bool {
    fn format(&self, _: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            true => "true",
            false => "false",
        };
        f.write_str(str)
    }
}

impl<T: RegistryDisplay> RegistryDisplay for Option<T> {
    fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(some) => {
                f.write_str("Some(")?;
                some.format(reg, f)?;
                f.write_char(')')
            }
            None => f.write_str("None"),
        }
    }
}

impl RegistryDisplay for Spur {
    fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*reg.resolve(self))
    }
}

impl RegistryDisplay for TypeDecl {
    fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        TypeDecl::fmt(self, f, reg)
    }
}

// impl<'a, I: RegistryDisplay, T: Iterator<Item = I> + Clone> RegistryDisplay for Separated<T> {
//     fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut iter = self.iter.clone().peekable();
//         let mut first = true;
//         while let Some(next) = iter.next() {
//             let last = iter.peek().is_none();
//             if !first {
//                 f.write_str(self.pad)?;
//             }
//             first = false;
//             RegistryDisplay::format(&next, reg, f)?;
//             if !last {
//                 f.write_str(self.end)?;
//                 if self.newline {
//                     f.write_char('\n')?;
//                 }
//             }
//         }
//         Ok(())
//     }
// }

impl<T: RegistryDisplay> RegistryDisplay for &T {
    fn format(&self, reg: &Registry, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::format(self, reg, f)
    }
}
