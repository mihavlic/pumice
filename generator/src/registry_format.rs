use generator_lib::{
    lasso::Spur,
    type_declaration::{fmt_tokens, Decl, RustDecl},
    Resolve,
};
use std::{
    cell::Cell,
    fmt::{Arguments, Display, Formatter, Write},
    ops::Deref,
};

use crate::Context;

pub struct GlobalContext<'a> {
    pub ctx: &'a Context,
    pub cur_section: Cell<u32>,
}

impl<'a> Deref for GlobalContext<'a> {
    type Target = Context;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

pub struct WithContext<'a, T: ?Sized>(&'a GlobalContext<'a>, T);

pub trait CtxWrap {
    fn wrp<'a>(self, ctx: &'a GlobalContext<'_>) -> WithContext<'a, Self>;
}

impl<T> CtxWrap for T {
    fn wrp<'a>(self, ctx: &'a GlobalContext<'_>) -> WithContext<'a, Self> {
        WithContext(ctx, self)
    }
}

pub trait RegistryDisplay {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<'a, T: RegistryDisplay> Display for WithContext<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RegistryDisplay::format(&self.1, self.0, f)
    }
}

impl RegistryDisplay for Arguments<'_> {
    fn format(&self, _: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(*self)
    }
}

// impl RegistryDisplay for &str {
//     fn format(&self, _: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.fmt(f)
//     }
// }

// impl RegistryDisplay for String {
//     fn format(&self, _: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.fmt(f)
//     }
// }

// impl RegistryDisplay for u32 {
//     fn format(&self, _: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}", self))
//     }
// }

// impl RegistryDisplay for bool {
//     fn format(&self, _: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.fmt(f)
//     }
// }

impl<T: RegistryDisplay> RegistryDisplay for Option<T> {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Some(some) => {
                f.write_str("Some(")?;
                some.format(ctx, f)?;
                f.write_char(')')
            }
            None => f.write_str("None"),
        }
    }
}

pub struct Pathed(pub Spur);

impl RegistryDisplay for Pathed {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_symbol_path(self.0, &ctx.ctx, ctx.cur_section.get(), f)
    }
}

impl RegistryDisplay for Spur {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.resolve(ctx).fmt(f)
    }
}

impl RegistryDisplay for RustDecl {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Decl {
            tokens, array_len, ..
        } = &self.0;

        let mut tokens = tokens.as_slice();
        if array_len.is_some() {
            tokens = &tokens[2..tokens.len()];
            f.write_char('[')?;
        }

        fmt_tokens(
            tokens,
            &|ident, f| fmt_symbol_path(ident, &ctx.ctx, ctx.cur_section.get(), f),
            f,
        )?;

        if let Some(size) = array_len {
            let size = size.resolve(ctx);
            if size.parse::<usize>().is_ok() {
                f.write_fmt(format_args!("; {}]", size))?;
            } else {
                f.write_fmt(format_args!("; {} as usize]", size))?;
            }
        }

        Ok(())
    }
}

impl<T: RegistryDisplay> RegistryDisplay for &T {
    fn format(&self, ctx: &GlobalContext, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::format(self, ctx, f)
    }
}

fn fmt_symbol_path(
    name: Spur,
    ctx: &Context,
    current_section: u32,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let name_ref = name.resolve(ctx);
    let str = match &*name_ref {
        // types always included from the platform module
        ffi @ ("void" | "char" | "float" | "double" | "size_t" | "int") => ffi,
        // just pass through primitive types
        native @ ("u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64") => native,
        "uint8_t" => "u8",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        "int8_t" => "i8",
        "int16_t" => "i16",
        "int32_t" => "i32",
        "int64_t" => "i64",
        other => {
            let section = ctx
                .get_item_section_idx(name)
                .unwrap_or_else(|| panic!("{}", &name_ref));
            if section != current_section {
                format_args!(
                    "crate::{}::",
                    ctx.sections()[section as usize].name.resolve(ctx)
                )
                .fmt(f)?;
            }
            other
        }
    };

    f.write_str(str)
}
