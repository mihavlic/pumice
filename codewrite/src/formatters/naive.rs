use std::error::Error;

use crate::WriteLast;

use super::Formatter;

#[repr(u8)]
enum State {
    Normal,
    SupressNewlines,
}

#[derive(PartialEq, Eq)]
enum Transient {
    Normal,
    Newline,
}

pub struct NaiveFormatter<W: std::fmt::Write> {
    w: W,
    stack: Vec<State>,
    transient: Transient,
    sep: bool,
    ident: u32,
}

impl<W: std::fmt::Write> NaiveFormatter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            w: writer,
            stack: vec![State::Normal],
            transient: Transient::Normal,
            sep: false,
            ident: 0,
        }
    }
}

impl<W: std::fmt::Write> std::fmt::Write for NaiveFormatter<W> {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        let Self {
            w,
            stack,
            transient,
            sep,
            ident,
        } = self;

        if *transient == Transient::Newline && matches!(stack.last(), Some(State::SupressNewlines))
        {
            *transient = Transient::Normal;
        }

        let mut t = Transient::Normal;
        let mut delayed_ident = 0;
        match c {
            '{' => {
                delayed_ident = 1;
                *sep = true;
                t = Transient::Newline;
                stack.push(State::Normal);
            }
            '}' => {
                *ident = ident.saturating_sub(1);
                *transient = Transient::Newline;
                t = Transient::Newline;
                stack.pop();
            }
            '(' => {
                stack.push(State::SupressNewlines);
            }
            ')' => {
                *sep = false;
                stack.pop();
            }
            '[' => {
                stack.push(State::SupressNewlines);
            }
            ']' => {
                *sep = false;
                stack.pop();
            }
            ';' => {
                *sep = false;
                t = Transient::Newline;
            }
            ',' => {
                *sep = false;
                t = Transient::Newline;
            }
            ' ' => {
                *sep = true;
                return Ok(());
            }
            '\n' => {
                if *transient == Transient::Newline {
                    write!(w, "\n")?;
                }

                *transient = Transient::Newline;
                return Ok(());
            }
            _ => {}
        };

        if *transient == Transient::Newline && matches!(stack.last(), Some(State::Normal)) {
            *sep = false;
            write!(w, "\n")?;
            for _ in 0..*ident {
                write!(w, "    ")?;
            }
        }

        *transient = t;
        *ident += delayed_ident;

        if *sep {
            *sep = false;
            write!(w, " ")?;
        }

        write!(w, "{c}")
    }
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }
}

impl<W: std::fmt::Write> Formatter for NaiveFormatter<W> {
    type Err = &'static dyn Error;
    // streaming formatter, no need to flush
    fn finish(self) -> Result<(), Self::Err> {
        Ok(())
    }
}
