use generator_lib::{type_declaration::TypeDecl, Interner};
use lasso::Spur;
use std::fmt::{Arguments, Display, Formatter, Write};

pub struct WithInterner<'a, T: ?Sized>(&'a Interner, T);

pub trait InternerWrap {
    fn int(self, reg: &Interner) -> WithInterner<Self>;
}

impl<T> InternerWrap for T {
    fn int(self, reg: &Interner) -> WithInterner<Self> {
        WithInterner(reg, self)
    }
}

pub trait RegistryDisplay {
    fn format(&self, reg: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<'a, T: RegistryDisplay> Display for WithInterner<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RegistryDisplay::format(&self.1, self.0, f)
    }
}

impl RegistryDisplay for Arguments<'_> {
    fn format(&self, _: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(*self)
    }
}

impl RegistryDisplay for &str {
    fn format(&self, _: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}

impl RegistryDisplay for String {
    fn format(&self, _: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl RegistryDisplay for u32 {
    fn format(&self, _: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl RegistryDisplay for bool {
    fn format(&self, _: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            true => "true",
            false => "false",
        };
        f.write_str(str)
    }
}

impl<T: RegistryDisplay> RegistryDisplay for Option<T> {
    fn format(&self, reg: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn format(&self, reg: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*reg.resolve(self))
    }
}

impl RegistryDisplay for TypeDecl {
    fn format(&self, reg: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        TypeDecl::fmt(self, f, reg)
    }
}

impl<T: RegistryDisplay> RegistryDisplay for &T {
    fn format(&self, reg: &Interner, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::format(self, reg, f)
    }
}
