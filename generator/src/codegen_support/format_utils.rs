use std::{
    fmt::Write,
    fs::OpenOptions,
    iter,
    ops::Deref,
    path::{Path, PathBuf},
    rc::Rc,
    thread::panicking,
};

use codewrite::{CFmt, WriteLast};
use generator_lib::{
    interner::UniqueStr,
    type_declaration::{fmt_type_tokens_impl, BasetypeOrRef, Type, TypeRef},
};

use crate::context::{Context, SectionFunctions, SectionIdent};

pub struct SectionWriter {
    pub section: SectionIdent,
    buf: String,
    pub ctx: Rc<Context>,
    pub path: PathBuf,
    pub append: bool,
}

impl WriteLast for SectionWriter {
    fn last_char(&self) -> Option<char> {
        self.buf.last_char()
    }
}

impl std::fmt::Write for SectionWriter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}

// impl ConcreteCFmtWriter for SectionWriter {
//     type Writer = SectionWriter;
//     type Context = Rc<Context>;
//     fn wctx<'a>(&'a mut self) -> (&'a mut Self::Writer, &'a mut Self::Context) {
//         (&mut self, &mut self.ctx)
//     }
// }

// pub trait SectionFmt: CFmt<SectionWriter> {}

impl SectionWriter {
    pub fn new(
        section: SectionIdent,
        path: impl AsRef<Path>,
        append: bool,
        ctx: &Rc<Context>,
    ) -> Self {
        Self {
            section,
            buf: String::new(),
            ctx: ctx.clone(),
            path: path.as_ref().to_path_buf(),
            append,
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        if self.buf.is_empty() {
            return Ok(());
        }

        let syn_file = syn::parse_file(&self.buf)
            .map_err(|e| {
                std::fs::write(&self.path, &self.buf).unwrap();
                panic!(
                    "Failed to parse syn file intended for '{}'\nErr: {}",
                    self.path.to_string_lossy(),
                    e
                )
            })
            .unwrap();

        let mut file = if self.append && self.path.exists() {
            OpenOptions::new()
                .append(true)
                .write(true)
                .open(&self.path)
                .map_err(|e| {
                    eprintln!("Error opening '{}'", self.path.to_string_lossy());
                    e
                })?
        } else {
            std::fs::File::create(&self.path).map_err(|e| {
                eprintln!("Error creating '{}'", self.path.to_string_lossy());
                e
            })?
        };

        use std::io::Write;
        let out = prettyplease::unparse(&syn_file);
        file.write_all(out.as_bytes())
    }
    pub fn pop_last_character(&mut self) {
        self.buf.pop();
    }
    pub fn last_character(&self) -> Option<char> {
        self.buf.chars().rev().next()
    }
    pub fn separate_idents(&mut self) {
        if self
            .last_character()
            .map(|c| !c.is_ascii_whitespace())
            .unwrap_or(false)
        {
            self.buf.push(' ');
        }
    }
}

impl Drop for SectionWriter {
    fn drop(&mut self) {
        if !panicking() {
            self.save().unwrap();
        }
    }
}

// #[macro_export]
// macro_rules! code {
//     ($($a:tt)*) => {
//         codewrite_macro::code!(
//             __qualify(crate::codegen_support::format_utils::SectionWriter, std::rc::Rc<crate::context::Context>)
//             $($a)*
//         )
//     };
// }

#[derive(Clone)]
pub struct Import<T>(pub T);

impl<T: Clone + Copy> Copy for Import<T> {}

#[macro_export]
macro_rules! import {
    ($e:expr) => {
        $crate::codegen_support::format_utils::Import($e)
    };
}

pub use import;

pub struct SymbolOrValue(pub UniqueStr);

impl CFmt<SectionWriter> for SymbolOrValue {
    fn cfmt(&self, w: &mut SectionWriter) {
        let str = self.0.resolve();
        if str.parse::<u64>().is_ok() {
            w.write_str(str).unwrap();
        } else {
            fmt_symbol_path(self.0, &w.section, &w.ctx, &mut w.buf).unwrap();
        }
    }
}

#[macro_export]
macro_rules! symbol_or_value {
    ($e:expr) => {
        $crate::codegen_support::format_utils::SymbolOrValue($e)
    };
}

pub use symbol_or_value;

#[macro_export]
macro_rules! import_str {
    ($e:expr, $ctx:expr) => {
        $crate::codegen_support::format_utils::Import(
            $e.try_intern($ctx)
                .unwrap_or_else(|| panic!("String '{}' is not in the interner!", $e)),
        )
    };
}

pub use import_str;

#[macro_export]
macro_rules! string {
    ($e:expr) => {
        codewrite::Concat([&'"', &$e, &'"'])
    };
}

pub use string;

#[macro_export]
macro_rules! cstring {
    ($e:expr) => {
        codewrite::Concat([&"crate::cstr!(\"", &$e, &"\")"])
    };
}

pub use cstring;

#[macro_export]
macro_rules! cat {
    ($($e:expr),+) => {
        codewrite::Concat([$(&$e),+])
    };
}

pub use cat;

#[macro_export]
macro_rules! cond {
    ($cond:expr => |$w:ident| $($body:tt)*) => {
        codewrite::Cond::new($cond, codewrite::FunOnce::new(|$w: &mut SectionWriter| { $($body)* }))
    };
}

pub use cond;

#[macro_export]
macro_rules! fun {
    ($($mov:ident)? |$w:ident| $($body:tt)*) => {
        codewrite::Fun::new($($mov)? |$w: &mut SectionWriter| { $($body)* })
    };
}

pub use fun;

#[macro_export]
macro_rules! fun_once {
    ($($mov:ident)? |$w:ident| $($body:tt)*) => {
        codewrite::FunOnce::new($($mov)? |$w: &mut SectionWriter| { $($body)* })
    };
}

pub use fun_once;

#[macro_export]
macro_rules! doc_comment {
    ($($e:expr),+) => {
        codewrite::Concat([&"///", $(& $e),+, &'\n'])
    };
}

pub use doc_comment;

#[macro_export]
macro_rules! doc_boilerplate {
    ($name:expr) => {
        codewrite::Fun::new(|w: &mut SectionWriter| {
            let original = $name.resolve_original();
            if !$name.is_original() {
                writeln!(w, r#"#[doc(alias = "{}")]"#, original).unwrap();
            }
            writeln!(w,"/// [Vulkan Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{}.html)", original).unwrap();
        })
    };
}

pub use doc_boilerplate;

impl CFmt<SectionWriter> for Import<UniqueStr> {
    fn cfmt(&self, w: &mut SectionWriter) {
        fmt_symbol_path(self.0, &w.section, &w.ctx, &mut w.buf).unwrap();
    }
}

impl CFmt<SectionWriter> for Import<&Type> {
    fn cfmt(&self, w: &mut SectionWriter) {
        Import(self.0.deref()).cfmt(w)
    }
}

impl CFmt<SectionWriter> for Import<&TypeRef> {
    fn cfmt(&self, w: &mut SectionWriter) {
        fmt_type_tokens_impl(
            self.0.as_slice(),
            &(|s, f: &mut String| fmt_symbol_path(s, &w.section, &w.ctx, f)),
            &(|s, f: &mut String| {
                // the array size is either a number literal or a constant potentially defined elsewhere
                let str = s.resolve();
                if str.parse::<u64>().is_ok() {
                    f.write_str(str)
                } else {
                    fmt_symbol_path(s, &w.section, &w.ctx, f)?;
                    f.write_str(" as usize")
                }
            }),
            &mut w.buf,
        )
        .unwrap();
    }
}
impl<'a> CFmt<SectionWriter> for Import<BasetypeOrRef<'a>> {
    fn cfmt(&self, w: &mut SectionWriter) {
        import!(self.0.deref()).cfmt(w)
    }
}

fn fmt_symbol_path(
    name: UniqueStr,
    current_section: &SectionIdent,
    ctx: &Context,
    w: &mut impl Write,
) -> std::fmt::Result {
    let s = &ctx.strings;
    crate::switch!(
        name;
        s.void, s.int, s.char, s.float, s.double => {
            write!(w, "std::os::raw::")?;
        };
        // an edge case for cstring constants because we're using `std::ffi::CStr` for them and not `*const c_char`
        s.__cstring_constant_type => {
            return write!(w, "&std::ffi::CStr");
        };
        // stdint.h types are always renamed to their rust-native counterparts and as such are always in scope
        s.bool, s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => {};
        s.usize, s.u8, s.u16, s.u32, s.u64, s.i8, s.i16, s.i32, s.i64 => unreachable!("Rust-native types shouldn't ever be used by our code!");
        @ {
            let section = ctx
                .symbol_get_section(name)
                .unwrap_or_else(|| panic!("Symbol '{}' is unowned by any section!", name));

            let mut foreign = false;

            if section.lib() != current_section.lib() {
                write!(w, "{}::", section.lib())?;
                foreign = true;
            } else if section.name() != current_section.name() {
                write!(w, "crate::")?;
                foreign = true;
            }

            if foreign {
                for path in section.path()
                    .chain(iter::once(section.name().resolve()).filter(|s| s != &"crate"))
                {
                    write!(w, "{}::", path)?;
                }
            }
        }
    );

    w.write_str(name.resolve())
}
