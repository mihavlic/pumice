use crate::{CFmt, WriteLast};

#[doc(hidden)]
pub trait InterpWriter: WriteLast + Sized {
    fn write_with_interpolations<C>(
        &mut self,
        ctx: &mut C,
        str: &str,
        interp: &[&dyn CFmt<Self, C>],
    ) {
        let mut interp = interp.into_iter();
        let mut chars = str.chars();
        while let Some(c) = chars.next() {
            if c == '\0' {
                self.separate_whitespace();
                interp.next().unwrap().cfmt(self, ctx);
                self.separate_whitespace();
            } else {
                self.write_char(c).unwrap();
            }
        }
    }
}

impl<T: WriteLast> InterpWriter for T {}
