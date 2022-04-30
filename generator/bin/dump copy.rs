use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use generator_lib::{
    format_utils::{RegistryFormat, RegistryWrap, Separated, WithRegistry},
    process_registry,
    type_declaration::TypeDecl,
    Registry, ToplevelBody, ToplevelKind,
};
use lasso::Spur;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Scope {
    Array,
    Struct,
    Anon,
    Transparent,
    Named,
}

struct Printer<'a, T: Write> {
    buf: T,
    reg: &'a Registry,
    indentation: u32,
    padding_unit: &'a str,
    scope: Vec<(Scope, bool)>,
}

impl<T: Write> Printer<'_, T> {
    fn write_padding(&mut self) {
        for _ in 0..self.indentation {
            self.buf.write(self.padding_unit.as_bytes()).unwrap();
        }
    }
    fn begin<C: RegistryFormat>(&mut self, name: Option<C>, scope: Scope) {
        self.member_prepare();
        if let Some(name) = name {
            write!(self.buf, "{}", name.reg(&self.reg)).unwrap();
        }
        if matches!(scope, Scope::Array | Scope::Struct) {
            self.buf.write(&[' ' as u8]).unwrap();
        }

        self.scope.push((scope, true));

        let char = match scope {
            Scope::Array => "[",
            Scope::Struct => "{",
            Scope::Anon => "(",
            Scope::Transparent | Scope::Named => return,
        };

        // transparent scope doesn't increase indent
        self.indentation += 1;
        self.buf.write(char.as_bytes()).unwrap();
    }
    fn end(&mut self) {
        if matches!(
            self.scope.last().unwrap().0,
            Scope::Transparent | Scope::Named
        ) {
            self.scope.pop();
            return;
        }

        self.indentation = self.indentation.checked_sub(1).unwrap();

        let end = match self.scope.pop().unwrap().0 {
            Scope::Array => "]",
            Scope::Struct => "}",
            Scope::Anon => ")",
            _ => unreachable!(),
        };
        self.buf.write("\n".as_bytes()).unwrap();
        self.write_padding();
        self.buf.write(end.as_bytes()).unwrap();
    }
    fn member_prepare(&mut self) {
        // this only occurs when setting up the topmost scope which by definition has no padding
        if let Some((scope, first)) = self.scope.last_mut() {
            if *scope == Scope::Named {
                return;
            }
            if *first == false {
                self.buf.write(",".as_bytes()).unwrap();
            }
            // this is a fragile for the topmost scope
            if !*first || *scope != Scope::Transparent {
                self.buf.write("\n".as_bytes()).unwrap();
            }
            *first = false;

            self.write_padding();
        }
    }
    fn named_begin<C: RegistryFormat>(&mut self, name: C) {
        self.member_prepare();
        write!(self.buf, "{}: ", name.reg(&self.reg)).unwrap();
        self.scope.push((Scope::Named, true));
    }
    fn member_simple<C: RegistryFormat, D: RegistryFormat>(&mut self, name: C, simple: D) {
        self.named_begin(name);
        // write!(self.buf, "{}", simple.reg(&self.reg)).unwrap();
        self.member_simple_anon(simple);
        self.end();
    }
    fn member_simple_anon<C: RegistryFormat>(&mut self, name: C) {
        self.member_prepare();
        write!(self.buf, "{}", name.reg(&self.reg)).unwrap();
    }
    fn slice_simple<C: RegistryFormat, I: Iterator<Item = C>>(&mut self, mut iter: I) {
        self.slice(iter, |s, i| s.member_simple_anon(i));
    }
    fn slice<C: RegistryFormat, I: Iterator<Item = C>, F: FnMut(&mut Self, C)>(
        &mut self,
        mut iter: I,
        mut fun: F,
    ) {
        self.member_prepare();
        let mut next = iter.next();
        if next.is_none() {
            self.buf.write("[]".as_bytes()).unwrap();
            return;
        }

        self.begin(Option::<&str>::None, Scope::Array);
        while let Some(some) = next.take() {
            fun(self, some);
            next = iter.next();
        }
        self.end();
    }
}

macro_rules! fields {
    ($formatter:ident, $source:expr,) => {};
    ($formatter:ident, $source:expr, $name:ident; $($tail:tt)*) => {
        $formatter.member_simple(stringify!($name), &$source.$name);
        fields!($formatter, $source, $($tail)*);
    };
    ($formatter:ident, $source:expr, $name:ident: $code:expr; $($tail:tt)*) => {
        $formatter.member_simple(stringify!($name), &($code));
        fields!($formatter, $source, $($tail)*);
    };
}

pub struct Wrap<T>(T);

pub trait ConstructWrapper: Sized {
    fn wrap(&self) -> Wrap<&Self> {
        Wrap(self)
    }
    fn wrap_owned(self) -> Wrap<Self> {
        Wrap(self)
    }
}

impl<T> ConstructWrapper for T {}

impl<T: RegistryFormat> RegistryFormat for Wrap<&Option<T>> {
    fn format(&self, reg: &Registry, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(some) => {
                f.write_str("Some(")?;
                some.format(reg, f)?;
                std::fmt::Write::write_char(f, ')')
            }
            None => f.write_str("None"),
        }
    }
}

impl RegistryFormat for Wrap<&Spur> {
    fn format(&self, reg: &Registry, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Write::write_char(f, '"')?;
        f.write_str(&*reg.resolve(&self.0))?;
        std::fmt::Write::write_char(f, '"')
    }
}

impl RegistryFormat for Wrap<&String> {
    fn format(&self, _: &Registry, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Write::write_char(f, '"')?;
        f.write_str(&self.0)?;
        std::fmt::Write::write_char(f, '"')
    }
}

impl RegistryFormat for Wrap<&ToplevelKind> {
    fn format(&self, _: &Registry, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.0 {
            ToplevelKind::Handle => "Handle",
            ToplevelKind::Constant => "Constant",
            ToplevelKind::Enum => "Enum",
            ToplevelKind::Bitmask => "Bitmask",
            ToplevelKind::Command => "Command",
            ToplevelKind::Alias => "Alias",
            ToplevelKind::Included => "Included",
            ToplevelKind::Basetype => "Basetype",
            ToplevelKind::Funcpointer => "Funcpointer",
            ToplevelKind::Struct => "Struct",
            ToplevelKind::BitmaskBits => "BitmaskBits",
        };
        f.write_str(name)
    }
}

impl<C: RegistryFormat, D: RegistryFormat> RegistryFormat for Wrap<(C, D)> {
    fn format(&self, reg: &Registry, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Write::write_char(f, '(')?;
        self.0 .0.format(reg, f)?;
        std::fmt::Write::write_str(f, ", ")?;
        self.0 .1.format(reg, f)?;
        std::fmt::Write::write_char(f, ')')
    }
}

#[rustfmt::skip]
fn main() {
    let (reg, errors) = vk_parse::parse_file(&Path::new("/home/eg/Downloads/vk.xml")).unwrap();
    if !errors.is_empty() {
        eprintln!("{:#?}", errors);
    }

    let bindgen_file = File::create(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../generated/dump.ron"
    ))
    .unwrap();

    let registry = process_registry(reg);
    let mut dst = Printer {
        buf: BufWriter::new(bindgen_file),
        reg: &registry,
        indentation: 0,
        padding_unit: "    ",
        scope: Vec::new(),
    };

    dst.begin(Option::<&str>::None, Scope::Transparent);

    for item in &registry.vendors {
        for c in &item.children {
            dst.begin(Some("VendorId"), Scope::Struct);
            fields!(
                dst, c,
                name: c.name.wrap();
                id;
            );
            dst.end();
        }
    }
    for item in &registry.platforms {
        for c in &item.children {
            dst.begin(Some("Platform"), Scope::Struct);
            fields!(
                dst, c,
                name: c.name.wrap();
                protect: c.protect.wrap();
            );
            dst.end();
        }
    }
    for item in &registry.tags {
        for c in &item.children {
            dst.begin(Some("Tag"), Scope::Struct);
            fields!(
                dst, c,
                name: c.name.wrap();
                author: c.author.wrap();
                contact: c.contact.wrap();
            );
            dst.end();
        }
    }
    for item in &registry.headers {
        dst.begin(Some("Header"), Scope::Anon);
        dst.member_simple_anon(item.wrap());
        dst.end();
    }
    for item in &registry.defines {
        dst.begin(Some("Define"), Scope::Struct);
        fields!(
            dst, item,
            name: item.name.wrap();
            body: item.body.wrap();
        );
        dst.end();
    }
    for item in &registry.toplevel {
        dst.begin(Some("Toplevel"), Scope::Struct);
        dst.member_simple("name", item.0);
        dst.named_begin("body");
        
        match &item.1 {
            ToplevelBody::Alias { alias_of, kind } => {
                dst.begin(Some("Alias"), Scope::Struct);
                fields!(
                    dst, item,
                    alias_of: alias_of.wrap();
                    kind: kind.wrap();
                );
            },
            ToplevelBody::Included { header } => {
                dst.begin(Some("Included"), Scope::Struct);
                fields!(
                    dst, item,
                    header: header.wrap();
                );
            },
            ToplevelBody::Basetype { ty, code } => {
                dst.begin(Some("Basetype"), Scope::Struct);
                fields!(
                    dst, item,
                    ty: ty.wrap();
                    code: code.wrap();
                );
            },
            ToplevelBody::Bitmask { ty, bits_enum } => {
                dst.begin(Some("Bitmask"), Scope::Struct);
                fields!(
                    dst, item,
                    ty: ty.wrap();
                    // Option<Spur> -> Wrap<&Option<Wrap<&Spur>>>
                    bits_enum: bits_enum.as_ref().map(|s| s.wrap()).wrap();
                );
            },
            ToplevelBody::Handle { object_type, dispatchable } => {
                dst.begin(Some("Handle"), Scope::Struct);
                fields!(
                    dst, item,
                    object_type: object_type;
                    dispatchable: dispatchable;
                );
            },
            ToplevelBody::Funcpointer { return_type, args } => {
                dst.begin(Some("Handle"), Scope::Struct);
                fields!(
                    dst, item,
                    return_type: return_type;
                );
                dst.named_begin("args");
                dst.slice_simple(args.iter().map(|a| (a.0.wrap(), &a.1).wrap_owned()
                ));
                dst.end();
            },
            ToplevelBody::Struct { union, members } => {
                dst.begin(Some("Struct"), Scope::Struct);
                fields!(
                    dst, item,
                    union: union;
                );
                dst.named_begin("members");
                dst.slice_simple(members.iter().map(|a| (a.0.wrap(), &a.1).wrap_owned()
                ));
                dst.end();
            },
            ToplevelBody::Constant { ty, val } => {
                dst.begin(Some("Constant"), Scope::Struct);
                fields!(
                    dst, item,
                    ty: ty.wrap();
                    val: val.wrap();
                );
            },
            ToplevelBody::Enum { members } => {
                dst.begin(Some("Struct"), Scope::Struct);
                dst.named_begin("members");

                dst.end();
            },
            ToplevelBody::BitmaskBits { members } => {

            },
            ToplevelBody::Command { return_type, params } => {

            },
            _ => {
                dst.begin(Some("Todo"), Scope::Struct);
            }
        }

        dst.end();
        dst.end();
        dst.end();
    }
    for item in &registry.features {

    }
    for item in &registry.extensions {

    }
    for item in &registry.formats {

    }
    for item in &registry.spirv_capabilities {

    }
    for item in &registry.spirv_extensions {

    }

    dst.end();

    // println!("{:#?}", registry.vendors);
    // println!("{:#?}", registry.platforms);
    // println!("{:#?}", registry.tags);
    // println!("{:#?}", registry.headers);
    // println!("{:#?}", registry.defines);

    // println!("{:#?}", registry.toplevel);
    // println!("{:#?}", registry.features);
    // println!("{:#?}", registry.extensions);
    // println!("{:#?}", registry.formats);
    // println!("{:#?}", registry.spirv_capabilities);
    // println!("{:#?}", registry.spirv_extensions);
}
