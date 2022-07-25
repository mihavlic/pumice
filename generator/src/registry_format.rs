use generator_lib::{
    interner::UniqueStr,
    type_declaration::{fmt_type_tokens_impl, Type},
};
use std::fmt::{Display, Formatter};

use crate::Context;

pub struct Pathed<'a, T>(pub T, pub &'a Context, pub u32);

impl<'a> Display for Pathed<'a, UniqueStr> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_symbol_path(self.0, self.1, self.2, f)
    }
}

impl<'a> Display for Pathed<'a, Type> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_type_tokens_impl(&self.0 .0, |s, f| fmt_symbol_path(s, &self.1, self.2, f), f)
    }
}

fn fmt_symbol_path(
    name: UniqueStr,
    ctx: &Context,
    current_section: u32,
    f: &mut Formatter<'_>,
) -> std::fmt::Result {
    let name_ref = name.resolve();
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
                    ctx.sections()[section as usize].name.resolve()
                )
                .fmt(f)?;
            }
            other
        }
    };

    f.write_str(str)
}
