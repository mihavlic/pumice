use std::{fmt::Write, mem::ManuallyDrop, thread::panicking};

use crate::WriteLast;

use super::{naive::NaiveFormatter, Formatter};

pub struct PrettyFormatter<W: std::fmt::Write> {
    w: W,
    buf: String,
}

impl<W: std::fmt::Write> std::fmt::Write for PrettyFormatter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}

impl<W: std::fmt::Write> PrettyFormatter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            w: writer,
            buf: String::new(),
        }
    }
}

pub fn format_pretty_or_fallback<W: Write>(src: &str, mut writer: W) -> syn::Result<()> {
    let syn_file = match syn::parse_file(&src) {
        Ok(f) => f,
        Err(mut e) => {
            // dump the raw string using at least the native formatting
            let _ = NaiveFormatter::new(writer).write_str(&src).map_err(|err| {
                e.extend(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("std::fmt::Write error: '{err}'"),
                ));
            });
            return Err(e);
        }
    };

    let string = prettyplease::unparse(&syn_file);
    writer.write_str(&string).map_err(|err| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("std::fmt::Write error: '{err}'"),
        )
    })
}

impl<W: std::fmt::Write> Formatter for PrettyFormatter<W> {
    type Err = syn::Error;
    fn finish(self) -> syn::Result<()> {
        let prison = ManuallyDrop::new(self);
        let mut writer = unsafe { std::ptr::read(&prison.w) };
        let buf = unsafe { std::ptr::read(&prison.buf) };

        format_pretty_or_fallback(&buf, &mut writer)
    }
}

impl<W: std::fmt::Write> Drop for PrettyFormatter<W> {
    fn drop(&mut self) {
        let err = unsafe { std::ptr::read(self).finish() };
        if !panicking() {
            err.unwrap();
        }
    }
}

impl<W: std::fmt::Write> WriteLast for PrettyFormatter<W> {
    fn last_char(&self) -> Option<char> {
        self.buf.last_char()
    }
}
