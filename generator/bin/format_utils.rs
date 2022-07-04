use std::fmt::{Display, Formatter, Write};

#[derive(PartialEq, Eq)]
enum State {
    Start,
    Whitespace,
    Newline,
}

// a state machine that does some simple formatting on text that is written to it
pub struct FormatWriter<W: Write> {
    inner: W,
    indent: usize,
    state: State,
}

impl<W: Write> Write for FormatWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for char in s.chars() {
            self.write_char(char);
        }
        Ok(())
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.write_char(c);
        Ok(())
    }
}

const IDENT_STR: &'static str = "    ";

impl<W: Write> FormatWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            inner: writer,
            indent: 0,
            state: State::Newline,
        }
    }
    pub fn write_char(&mut self, c: char) {
        // match situations where the state doesn't change and nothing is written
        if (c.is_whitespace() && (self.state == State::Whitespace || self.state == State::Newline))
            || (c == '\n' && self.state == State::Newline)
        {
            return;
        }

        match c {
            '{' => {
                self.indent += 1;
                self.state = State::Start;
            }
            '}' => {
                // if this number underverflows during a release build it will
                // result in 2^64 indents being printed and, well, that's not good
                self.indent = self.indent.saturating_sub(1);
                self.state = State::Start;
            }
            _ => {}
        }

        match c {
            '\n' => {
                if self.state != State::Newline {
                    self.state = State::Newline;
                }
            }
            _ if c.is_whitespace() => {
                // State::Newline cannot be overriden by State::Whitespace
                if self.state != State::Newline {
                    self.state = State::Whitespace;
                    // we delay printing the indentation to the next character in case it's
                    // a brace that would modify the indentation before itself
                }
            }
            _ => {
                if self.state == State::Newline {
                    for _ in 0..self.indent {
                        self.write_raw(IDENT_STR);
                    }
                }
                self.state = State::Start;
            }
        }

        self.write_char_raw(c);
    }
    pub fn flush_newline(&mut self) {
        if self.state != State::Newline {
            self.write_char('\n');
        }
    }
    pub fn write_char_raw(&mut self, c: char) {
        self.inner.write_char(c).unwrap();
    }
    pub fn write_raw(&mut self, s: &str) {
        self.inner.write_str(s).unwrap();
    }
    pub fn get_inner_writer(&self) -> &W {
        &self.inner
    }
    pub fn to_inner_writer(self) -> W {
        self.inner
    }
}

pub struct WriteWriteAdapter<W: std::io::Write> (pub W);

impl<W: std::io::Write> Write for WriteWriteAdapter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.0.write(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }    
}

#[macro_export]
macro_rules! code2 {
    ($buf:expr, $($code:literal)+ @ $($tail:tt)*) => {
        {
            let result = std::fmt::Write::write_fmt(
                &mut $buf,
                format_args!(
                    concat!($($code, "\n"),+),
                    $(
                        $tail
                    )*
                )
            );
            if result.is_ok() {
                $buf.flush_newline();
            }
            result
        }
    }
}

pub struct Separated<
    'a,
    T: Iterator + Clone,
    F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result,
> {
    iter: T,
    fun: F,
    sep: &'a str,
    // whether to add a separator after the last element
    sep_last: bool,
}


impl<
        'a,
        T: Iterator + Clone,
        F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result,
    > Separated<'a, T, F>
{ 
    pub fn new(
        iter: T,
        fun: F,
        separator: &'a str,
        separator_last: bool        
    ) -> Self {
        Self { iter, fun, sep: separator, sep_last: separator_last }
    }
    pub fn args(
        iter: T,
        fun: F        
    ) -> Self {
        Self::new(iter, fun, ", ", false)
    }
    pub fn members(
        iter: T,
        fun: F        
    ) -> Self {
        Self::new(iter, fun, ",\n", false)
    }
    pub fn statements(
        iter: T,
        fun: F        
    ) -> Self {
        Self::new(iter, fun, ";\n", true)
    }
}

impl<
        'a,
        T: Iterator + Clone,
        F: Fn(<T as Iterator>::Item, &mut Formatter<'_>) -> std::fmt::Result,
    > Display for Separated<'a, T, F>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter.clone().peekable();

        while let Some(next) = iter.next() {
            (self.fun)(next, f)?;

            if iter.peek().is_some() || self.sep_last {
                f.write_str(self.sep)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_format_writer() {
    #[rustfmt::skip]
    let raw = 
r#"
struct {
            a;
a;

}
fn test(a: usize) {
    // comment
}
"#;

    #[rustfmt::skip]
    let expect = 
r#"struct {
    a;
    a;
}
fn test(a: usize) {
    // comment
}
"#;

    let mut writer = FormatWriter::new(String::new());
    writer.write_str(raw).unwrap();

    assert_eq!(writer.get_inner_writer(), expect);
}
