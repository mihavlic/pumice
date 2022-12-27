use std::{error::Error, io::Write};

use super::Formatter;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum State {
    Newline,
    Start,
    Ansi,
}

// a state machine that does some simple formatting on text that is written to it
pub struct WidthAnsiFormatter<W> {
    w: W,
    max_width: u16,
    width: u16,
    padding: &'static str,
    padding_width: u16,
    state: State,
    deffered: State,
}

impl<W: Write> WidthAnsiFormatter<W> {
    pub fn new(writer: W, mut padding: &'static str) -> Self {
        let mut padding_width = display_width(padding).try_into().unwrap();
        let max_width = termsize::get().map(|s| s.cols).unwrap_or(80);

        if padding_width >= max_width {
            padding = "";
            padding_width = 0;
        }

        Self {
            w: writer,
            max_width,
            width: 0,
            padding,
            padding_width,
            state: State::Start,
            deffered: State::Start,
        }
    }
}

const CSI: (char, char) = ('\x1b', '[');

impl<W: Write> std::io::Write for WidthAnsiFormatter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut iter = std::str::from_utf8(buf)
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "WidthAnsiFormatter received non-utf8 bytes!",
                )
            })?
            .chars();

        while let Some(next) = iter.next() {
            match next {
                '\x40'..='\x7e' if self.state == State::Ansi => {
                    self.state = self.deffered;
                }
                '\n' => {
                    self.state = State::Newline;
                    self.width = 0;
                }
                _ => {
                    if self.state == State::Newline {
                        self.width = self.padding_width;
                        self.w.write_all(self.padding.as_bytes())?;
                    }
                    self.state = State::Start;
                }
            }

            if next == CSI.0 && iter.clone().next() == Some(CSI.1) {
                self.deffered = self.state;
                self.state = State::Ansi;
                iter.next();
                self.w.write_all(b"\x1b[")?;
                continue;
            }

            let tmp = &mut [0u8; 8];
            let str = next.encode_utf8(tmp);
            self.w.write_all(str.as_bytes())?;

            if self.state != State::Ansi {
                self.width += 1;
                if self.width == self.max_width {
                    self.w.write_all(b"\n")?;
                    self.state = State::Newline;
                    self.width = 0;
                }
            }
        }

        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }
}

impl<W: Write> std::fmt::Write for WidthAnsiFormatter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.write(s.as_bytes())
            .map(|_| {})
            .map_err(|_| std::fmt::Error)
    }
}

pub fn display_width(str: &str) -> usize {
    let mut i = 0;
    let mut chars = str.chars();
    'outer: while let Some(next) = chars.next() {
        if next == CSI.0 && chars.clone().next() == Some(CSI.1) {
            chars.next();
            while let Some(next) = chars.next() {
                if let '\x40'..='\x7e' = next {
                    continue 'outer;
                }
            }
        }
        i += 1;
    }
    i
}

impl<W: std::io::Write> Formatter for WidthAnsiFormatter<W> {
    type Err = &'static dyn Error;
    // streaming formatter, no need to flush
    fn finish(self) -> Result<(), Self::Err> {
        Ok(())
    }
}
