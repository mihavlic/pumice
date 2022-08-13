use std::{
    fmt::{Display, Write},
    fs::OpenOptions,
    iter,
    path::{Path, PathBuf},
    thread::panicking,
};

use generator_lib::{
    interner::UniqueStr,
    type_declaration::{fmt_type_tokens_impl, Type},
};

use crate::{Context, Section};

pub struct SectionWriter<'a> {
    pub buf: String,
    pub section: u32,
    pub ctx: &'a Context,
    pub path: PathBuf,
    pub ends_whitespace: bool,
    pub append: bool,
}

impl<'a> std::fmt::Write for SectionWriter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if let Some(last) = s.chars().last() {
            self.ends_whitespace = last.is_ascii_whitespace();
        }
        self.buf.write_str(s)
    }
}

impl<'a> SectionWriter<'a> {
    pub fn new(section: u32, path: impl AsRef<Path>, append: bool, ctx: &'a Context) -> Self {
        Self {
            buf: String::new(),
            section,
            ctx,
            path: path.as_ref().to_path_buf(),
            ends_whitespace: true,
            append,
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
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

        let mut file = if self.append {
            OpenOptions::new()
                .append(true)
                .write(true)
                .open(&self.path)?
        } else {
            std::fs::File::create(&self.path)?
        };

        use std::io::Write;
        let out = prettyplease::unparse(&syn_file);
        file.write_all(out.as_bytes())
    }
    pub fn separate_idents(&mut self) {
        if !self.ends_whitespace {
            self.buf.push(' ');
            self.ends_whitespace = true;
        }
    }
}

impl<'a> Drop for SectionWriter<'a> {
    fn drop(&mut self) {
        if !panicking() {
            self.save().unwrap();
        }
    }
}

pub trait ExtendedFormat {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result;
}

impl<T: Display> ExtendedFormat for T {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        write!(writer, "{}", self)
    }
}

pub struct Fun<T: FnOnce(&mut SectionWriter<'_>) -> std::fmt::Result>(pub T);

impl<T: FnOnce(&mut SectionWriter<'_>) -> std::fmt::Result> ExtendedFormat for Fun<T> {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        (self.0)(writer)
    }
}

pub struct Cond<T: ExtendedFormat>(pub bool, pub T);

impl<T: ExtendedFormat> ExtendedFormat for Cond<T> {
    fn efmt(self, writer: &mut SectionWriter<'_>) -> std::fmt::Result {
        let Cond(cond, t) = self;
        if cond {
            t.efmt(writer)
        } else {
            Ok(())
        }
    }
}

pub struct Iter<T, I: IntoIterator<Item = T>, F: Fn(&mut SectionWriter<'_>, T)>(pub I, pub F);

impl<T, I: IntoIterator<Item = T>, F: Fn(&mut SectionWriter<'_>, T)> ExtendedFormat
    for Iter<T, I, F>
{
    fn efmt(self, writer: &mut SectionWriter<'_>) -> std::fmt::Result {
        for it in self.0 {
            (self.1)(writer, it);
        }
        Ok(())
    }
}

pub struct Concat<'a, const S: usize>(pub [&'a dyn Display; S]);

impl<'a, const S: usize> ExtendedFormat for &Concat<'a, S> {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        for it in self.0 {
            write!(writer, "{}", it)?;
        }
        Ok(())
    }
}

pub struct Separated<
    'a,
    T: IntoIterator,
    F: Fn(&mut SectionWriter<'_>, <T as IntoIterator>::Item) -> std::fmt::Result,
> {
    iter: T::IntoIter,
    fun: F,
    sep: &'a str,
    // whether to add a separator after the last element
    sep_last: bool,
}

impl<
        'a,
        T: IntoIterator,
        F: Fn(&mut SectionWriter<'_>, <T as IntoIterator>::Item) -> std::fmt::Result,
    > Separated<'a, T, F>
{
    pub fn new(iter: T, fun: F, separator: &'a str, separator_last: bool) -> Self {
        Self {
            iter: iter.into_iter(),
            fun,
            sep: separator,
            sep_last: separator_last,
        }
    }
    pub fn args(iter: T, fun: F) -> Self {
        Self::new(iter, fun, ",", false)
    }
    pub fn statements(iter: T, fun: F) -> Self {
        Self::new(iter, fun, ";", true)
    }
}

impl<'a, T: IntoIterator>
    Separated<'a, T, fn(&mut SectionWriter<'_>, <T as IntoIterator>::Item) -> std::fmt::Result>
where
    <T as IntoIterator>::Item: ExtendedFormat,
{
    pub fn display(iter: T, separator: &'a str) -> Self {
        Self::new(
            iter,
            |w: &mut SectionWriter<'_>, a: <T as IntoIterator>::Item| a.efmt(w),
            separator,
            false,
        )
    }
}

impl<
        'a,
        T: IntoIterator,
        F: Fn(&mut SectionWriter<'_>, <T as IntoIterator>::Item) -> std::fmt::Result,
    > ExtendedFormat for Separated<'a, T, F>
{
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        let mut iter = self.iter.peekable();

        while let Some(next) = iter.next() {
            (self.fun)(writer, next)?;

            if iter.peek().is_some() || self.sep_last {
                writer.write_str(self.sep)?;
            }
        }
        Ok(())
    }
}

pub struct Import<T>(pub T);

#[macro_export]
macro_rules! import {
    ($e:expr) => {
        $crate::format_utils::Import($e)
    };
}

pub use import;

#[macro_export]
macro_rules! string {
    ($e:expr) => {
        $crate::format_utils::Concat([&'"', &$e, &'"'])
    };
}

pub use string;

#[macro_export]
macro_rules! cstring {
    ($e:expr) => {
        $crate::format_utils::Concat([&"cstr!(\"", &$e, &"\")"])
    };
}

pub use cstring;

#[macro_export]
macro_rules! cat {
    ($($e:expr),+) => {
        $crate::format_utils::Concat([$(& $e),+])
    };
}

pub use cat;

#[macro_export]
macro_rules! doc_comment {
    ($($e:expr),+) => {
        $crate::format_utils::Concat([&"///", $(& $e),+, &'\n'])
    };
}

pub use doc_comment;

#[macro_export]
macro_rules! doc_boilerplate {
    ($name:expr) => {
        $crate::format_utils::Fun(|w| {
            let original = $name.resolve_original();
            writeln!(w,"/// [{}](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{}.html)", original, original)?;
            if !$name.is_original() {
                writeln!(w,r#"#[doc(alias = "{}")]"#, original)?;
            }
            Ok(())
        })
    };
}

pub use doc_boilerplate;

impl ExtendedFormat for Import<UniqueStr> {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        fmt_symbol_path(self.0, &writer.ctx, writer.section, &mut writer.buf)
    }
}

impl ExtendedFormat for Import<&Type> {
    fn efmt(self, w: &mut SectionWriter) -> std::fmt::Result {
        fmt_type_tokens_impl(
            &self.0 .0,
            &(|s, f: &mut String| fmt_symbol_path(s, &w.ctx, w.section, f)),
            &(|s, f| {
                // the array size is either a number literal or a constant potentially defined elsewhere
                let str = s.resolve();
                if str.parse::<u64>().is_ok() {
                    f.write_str(str)
                } else {
                    fmt_symbol_path(s, &w.ctx, w.section, f)?;
                    f.write_str(" as usize")
                }
            }),
            &mut w.buf,
        )
    }
}

fn fmt_symbol_path(
    name: UniqueStr,
    ctx: &Context,
    current_section: u32,
    f: &mut impl Write,
) -> std::fmt::Result {
    let s = &ctx.strings;
    crate::switch!(
        name;
        // these ffi types are imported into every module
        s.void, s.int, s.char, s.float, s.double,
        // stdint.h types are always renamed to their rust-native counterparts
        s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => {};
        s.usize, s.u8, s.u16, s.u32, s.u64, s.i8, s.i16, s.i32, s.i64 => unreachable!("Rust-native types shouldn't ever be used by our code!");
        @ {
            let section_idx = ctx
                .symbol_get_section_idx(name)
                .unwrap_or_else(|| panic!("{}", name));

            if section_idx != current_section {
                f.write_str("crate::")?;

                let section = &ctx.sections[section_idx as usize];
                let section_name = section.name.resolve();

                for path in section_get_path(section)
                    .iter()
                    .chain(iter::once(&section_name))
                {
                    write!(f, "{}::", path)?;
                }
            }
        }
    );

    f.write_str(name.resolve())
}

pub fn section_get_path(section: &Section) -> &'static [&'static str] {
    match section.kind {
        crate::SectionKind::Feature(_) => &[],
        crate::SectionKind::Extension(_) => &["extensions"],
    }
}

pub fn section_get_path_str(section: &Section) -> &'static str {
    match section.kind {
        crate::SectionKind::Feature(_) => "",
        crate::SectionKind::Extension(_) => "/extensions",
    }
}
