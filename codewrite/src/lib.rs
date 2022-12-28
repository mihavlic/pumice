pub mod formatters;
pub mod write;

use std::{
    cell::Cell,
    fmt::{Display, Write},
    marker::PhantomData,
};

pub trait WriteLast: Write {
    fn last_char(&self) -> Option<char>;
    fn separate_whitespace(&mut self) {
        if let Some(last) = self.last_char() {
            if !last.is_ascii_whitespace() {
                self.write_char(' ').unwrap();
            }
        }
    }
}

pub struct WriteLastWrapper<W: Write>(W, Option<char>);

impl<W: Write> WriteLast for WriteLastWrapper<W> {
    fn last_char(&self) -> Option<char> {
        self.1
    }
}

impl WriteLast for String {
    fn last_char(&self) -> Option<char> {
        self.chars().rev().next()
    }
}

impl<W: Write> Write for WriteLastWrapper<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if !s.is_empty() {
            self.1 = Some(s.chars().rev().next().unwrap());
            self.0.write_str(s)
        } else {
            Ok(())
        }
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.1 = Some(c);
        self.0.write_char(c)
    }
}

// strings in particular have a hard time becoming &dyn
#[doc(hidden)]
pub trait CFmtCoerce {
    fn coerce<T: WriteLast>(&self) -> &dyn CFmt<T>
    where
        Self: CFmt<T>;
}

impl<A> CFmtCoerce for A {
    fn coerce<T: WriteLast>(&self) -> &dyn CFmt<T>
    where
        Self: CFmt<T>,
    {
        self
    }
}

pub trait CFmt<T: WriteLast + ?Sized> {
    fn cfmt(&self, w: &mut T);
}

impl<T: WriteLast> CFmt<T> for str {
    fn cfmt(&self, w: &mut T) {
        write!(w, "{self}").unwrap();
    }
}

impl<T: WriteLast, D: Display> CFmt<T> for D {
    fn cfmt(&self, w: &mut T) {
        write!(w, "{self}").unwrap();
    }
}

pub struct Fun<T: WriteLast, F: Fn(&mut T)>(F, PhantomData<*const T>);

impl<T: WriteLast, F: Fn(&mut T)> Fun<T, F> {
    pub fn new(fun: F) -> Self {
        Self(fun, PhantomData)
    }
}

impl<T: WriteLast, F: Fn(&mut T)> CFmt<T> for Fun<T, F> {
    fn cfmt(&self, w: &mut T) {
        (self.0)(w)
    }
}

pub struct FunOnce<T: WriteLast, F: FnOnce(&mut T)>(SingleCell<F>, PhantomData<*const T>);

impl<T: WriteLast, F: FnOnce(&mut T)> FunOnce<T, F> {
    pub fn new(fun: F) -> Self {
        Self(SingleCell::new(fun), PhantomData)
    }
}

impl<T: WriteLast, F: FnOnce(&mut T)> CFmt<T> for FunOnce<T, F> {
    fn cfmt(&self, w: &mut T) {
        (self.0.get())(w)
    }
}

#[derive(Clone)]
pub struct Cond<T: WriteLast, F: CFmt<T>>(pub bool, pub F, PhantomData<*const T>);

impl<T: WriteLast, F: CFmt<T>> Cond<T, F> {
    pub fn new(cond: bool, val: F) -> Self {
        Self(cond, val, PhantomData)
    }
}

impl<T: WriteLast, F: CFmt<T>> CFmt<T> for Cond<T, F> {
    fn cfmt(&self, w: &mut T) {
        let Cond(cond, val, _) = self;
        if *cond {
            val.cfmt(w)
        }
    }
}

struct SingleCell<T>(Cell<Option<T>>);

impl<T: Copy> Clone for SingleCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SingleCell<T> {
    fn new(val: T) -> Self {
        Self(Cell::new(Some(val)))
    }
    fn get(&self) -> T {
        self.0.take().unwrap()
    }
}

pub struct Iter<T: WriteLast, I: IntoIterator, F: FnMut(&mut T, I::Item)>(
    SingleCell<(I, F)>,
    PhantomData<*const T>,
);

impl<T: WriteLast, I: IntoIterator, F: FnMut(&mut T, I::Item)> Iter<T, I, F> {
    pub fn new(iter: I, fun: F) -> Self {
        Self(SingleCell::new((iter, fun)), PhantomData)
    }
}

impl<T: WriteLast, I: IntoIterator, F: FnMut(&mut T, I::Item)> CFmt<T> for Iter<T, I, F> {
    fn cfmt(&self, w: &mut T) {
        let (iter, mut fun) = self.0.get();
        for it in iter {
            fun(w, it);
        }
    }
}

#[repr(transparent)]
pub struct Concat<'a, const S: usize>(pub [&'a dyn Display; S]);

impl<'a, const S: usize> Display for Concat<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &it in &self.0 {
            write!(f, "{it}")?;
        }
        Ok(())
    }
}

pub struct Separated<'a, T: WriteLast, I: IntoIterator, F: Fn(&mut T, <I as IntoIterator>::Item)> {
    iter_fun: SingleCell<(I, F)>,
    sep: &'a str,
    // whether to add a separator after the last element
    sep_last: bool,
    spooky: std::marker::PhantomData<T>,
}

impl<'a, I: IntoIterator, T: WriteLast, F: Fn(&mut T, <I as IntoIterator>::Item)> Clone
    for Separated<'a, T, I, F>
where
    I: Copy,
    F: Copy,
{
    fn clone(&self) -> Self {
        Self {
            iter_fun: self.iter_fun.clone(),
            sep: self.sep.clone(),
            sep_last: self.sep_last.clone(),
            spooky: std::marker::PhantomData,
        }
    }
}

impl<'a, I: IntoIterator, T: WriteLast, F: Fn(&mut T, <I as IntoIterator>::Item)>
    Separated<'a, T, I, F>
{
    pub fn new(iter: I, fun: F, separator: &'a str, separator_last: bool) -> Self {
        Self {
            iter_fun: SingleCell::new((iter, fun)),
            sep: separator,
            sep_last: separator_last,
            spooky: std::marker::PhantomData,
        }
    }
    pub fn args(iter: I, fun: F) -> Self {
        Self::new(iter, fun, ",", false)
    }
    pub fn statements(iter: I, fun: F) -> Self {
        Self::new(iter, fun, ";", true)
    }
}

impl<'a, T: WriteLast, I: IntoIterator> Separated<'a, T, I, fn(&mut T, <I as IntoIterator>::Item)>
where
    <I as IntoIterator>::Item: CFmt<T>,
{
    pub fn display(iter: I, separator: &'a str) -> Self {
        Self::new(
            iter,
            |w: &mut T, a: <I as IntoIterator>::Item| a.cfmt(w),
            separator,
            false,
        )
    }
}

impl<'a, T: WriteLast, I: IntoIterator, F: Fn(&mut T, <I as IntoIterator>::Item)> CFmt<T>
    for Separated<'a, T, I, F>
{
    fn cfmt(&self, w: &mut T) {
        let (iter, fun) = self.iter_fun.get();
        let mut iter = iter.into_iter().peekable();
        while let Some(next) = iter.next() {
            fun(w, next);
            if iter.peek().is_some() || self.sep_last {
                w.write_str(self.sep).unwrap();
            }
        }
    }
}

pub struct Format2Format<W: std::io::Write>(pub W);

impl<W: std::io::Write> std::fmt::Write for Format2Format<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}
