use crate::{CFmt, WriteLast};

pub trait CodewriteImpl: WriteLast {
    #[doc(hidden)]
    fn write_with_interpolations(&mut self, str: &str, interp: &[&dyn CFmt<Self>]) {
        let mut interp = interp.into_iter();
        let mut chars = str.chars();
        while let Some(c) = chars.next() {
            if c == '\0' {
                self.separate_whitespace();
                interp.next().unwrap().cfmt(self);
                self.separate_whitespace();
            } else {
                self.write_char(c).unwrap();
            }
        }
    }
}

impl<T: WriteLast> CodewriteImpl for T {}
