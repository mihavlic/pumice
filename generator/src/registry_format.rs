use generator_lib::{lasso::Spur, type_declaration::RustDecl};
use std::fmt::{Arguments, Display, Formatter, Write};

use crate::Context;

pub struct WithContext<'a, T: ?Sized>(&'a Context, T);

pub trait CtxWrap {
    fn ctx(self, ctx: &Context) -> WithContext<Self>;
}

impl<T> CtxWrap for T {
    fn ctx(self, ctx: &Context) -> WithContext<Self> {
        WithContext(ctx, self)
    }
}

pub trait RegistryDisplay {
    fn format(&self, reg: &Context, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<'a, T: RegistryDisplay> Display for WithContext<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RegistryDisplay::format(&self.1, self.0, f)
    }
}

impl RegistryDisplay for Arguments<'_> {
    fn format(&self, _: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(*self)
    }
}

impl RegistryDisplay for &str {
    fn format(&self, _: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

impl RegistryDisplay for String {
    fn format(&self, _: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RegistryDisplay for u32 {
    fn format(&self, _: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl RegistryDisplay for bool {
    fn format(&self, _: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            true => "true",
            false => "false",
        };
        f.write_str(str)
    }
}

impl<T: RegistryDisplay> RegistryDisplay for Option<T> {
    fn format(&self, reg: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn format(&self, reg: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*reg.resolve(self))
    }
}

impl RegistryDisplay for RustDecl {
    fn format(&self, reg: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        RustDecl::fmt(self, f, reg)
    }
}

impl<T: RegistryDisplay> RegistryDisplay for &T {
    fn format(&self, reg: &Context, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::format(self, reg, f)
    }
}
