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

impl<W: std::fmt::Write> Formatter for PrettyFormatter<W> {
    type Err = syn::Error;
    fn finish(self) -> Result<(), Self::Err> {
        let mut prison = ManuallyDrop::new(self);

        let syn_file = match syn::parse_file(&prison.buf) {
            Ok(f) => f,
            Err(e) => {
                let w = unsafe { std::ptr::read(&prison.w) };
                let buf = unsafe { std::ptr::read(&prison.buf) };
                // dump the raw string using at least the native formatting
                NaiveFormatter::new(w).write_str(&buf).unwrap();
                return Err(e);
            }
        };

        let string = prettyplease::unparse(&syn_file);
        prison.w.write_str(&string).unwrap();
        Ok(())
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
