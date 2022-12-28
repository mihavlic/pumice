use std::error::Error;

use super::Formatter;

#[derive(PartialEq)]
#[repr(u8)]
enum State {
    String,
    Comment,
    Normal,
    SupressNewlines,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Transient {
    Normal,
    Newline,
    NewlineWish,
}

pub struct NaiveFormatter<W: std::fmt::Write> {
    w: W,
    stack: Vec<State>,
    transient: Transient,
    sep: bool,
    ident: u32,
    prev_slash: bool,
    prev_backslash: bool,
}

impl<W: std::fmt::Write> NaiveFormatter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            w: writer,
            stack: vec![State::Normal],
            transient: Transient::Normal,
            sep: false,
            ident: 0,
            prev_slash: false,
            prev_backslash: false,
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
            prev_slash,
            prev_backslash,
        } = self;

        if *transient == Transient::Newline && matches!(stack.last(), Some(State::SupressNewlines))
        {
            *transient = Transient::Normal;
        }

        if matches!(stack.last(), Some(State::Comment)) {
            if c == '\n' {
                stack.pop();
            } else {
                return write!(w, "{c}");
            }
        }

        if matches!(stack.last(), Some(State::String)) {
            if c == '\\' {
                *prev_backslash = true;
            } else {
                *prev_backslash = false;
            }

            if c == '"' && !*prev_backslash {
                stack.pop();
            } else {
                return write!(w, "{c}");
            }
        }

        let mut t = Transient::Normal;
        let mut delayed_ident = 0;
        let mut fresh_stack = false;
        let mut saw_slash = false;
        let mut saw_backslash = false;
        let mut sep_after = false;
        match c {
            '{' => {
                *sep = false;
                delayed_ident = 1;
                *sep = true;
                t = Transient::NewlineWish;
                fresh_stack = true;
                stack.push(State::Normal);
            }
            '}' => {
                *sep = false;
                *ident = ident.saturating_sub(1);
                t = Transient::NewlineWish;
                stack.pop();
            }
            '(' => {
                *sep = false;
                fresh_stack = true;
                stack.push(State::SupressNewlines);
            }
            ')' => {
                *sep = false;
                stack.pop();
            }
            '[' => {
                *sep = false;
                fresh_stack = true;
                stack.push(State::SupressNewlines);
            }
            ']' => {
                *sep = false;
                t = Transient::NewlineWish;
                stack.pop();
            }
            ';' => {
                *sep = false;
                t = Transient::NewlineWish;
            }
            ':' => {
                *sep = false;
            }
            ',' => {
                *sep = false;
                sep_after = true;
                t = Transient::NewlineWish;
            }
            '=' => {
                *sep = true;
                sep_after = true;
            }
            '!' => {
                *sep = false;
            }
            '\n' => {
                if *transient == Transient::Newline {
                    write!(w, "\n")?;
                }

                *transient = Transient::Newline;
                return Ok(());
            }
            c if c.is_ascii_whitespace() => {
                *sep = true;
                return Ok(());
            }
            '/' => {
                saw_slash = true;
                if *prev_slash && saw_slash && stack.last() != Some(&State::Comment) {
                    fresh_stack = true;
                    stack.push(State::Comment);
                }
            }
            '"' => {
                if !*prev_backslash {
                    fresh_stack = true;
                    stack.push(State::String);
                }
            }
            '\\' => {
                saw_backslash = true;
            }
            _ => {}
        };

        *prev_slash = saw_slash;
        *prev_backslash = saw_backslash;

        let do_newline = match (*transient, stack.last(), fresh_stack) {
            (Transient::Newline | Transient::NewlineWish, Some(State::Normal), _)
            | (Transient::Newline | Transient::NewlineWish, _, true) => true,
            (Transient::Newline, Some(State::Comment), _) => true,
            _ => false,
        };

        if do_newline {
            *sep = false;
            *prev_slash = false;
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
        if sep_after {
            *sep = true;
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
