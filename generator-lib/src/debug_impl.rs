use std::{
    cell::Cell,
    fmt::{Debug, Write},
};

use lasso::Spur;

use crate::{
    type_declaration::{CDecl},
    CommandParameter, Component, ConstantValue, EnumValue, ExtendMethod, Extension, Feature,
    FeatureExtensionItem, Format, InterfaceItem, Interner, NumericFormat, Plane, Platform,
    Registry, SpirvEnable, SpirvExtCap, Tag, Toplevel, ToplevelBody, YCBREncoding,
};

// ew, cursed thing that formats owned iterators as slices
// since iteration mutates the iterator but Display operates on immutable self we need
// interior mutability, this is unlikely to break but is still discusting
struct SliceDebug<T>(T);

impl<T: Iterator> SliceDebug<Cell<Option<T>>> {
    fn new(iter: impl IntoIterator<IntoIter = T>) -> Self {
        Self(Cell::new(Some(iter.into_iter())))
    }
}

impl<T: Debug, I: Iterator<Item = T>> Debug for SliceDebug<Cell<Option<I>>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.take().unwrap()).finish()
    }
}

// duplicated from format_utils because we want the correct Debug behaviour
// such as emitting "" for strings and the like
struct WithInterner<'a, T: ?Sized>(&'a Interner, T);

trait InternerWrap {
    fn wrp<'a>(&'a self, reg: &'a Interner) -> WithInterner<&'a Self>;
}

impl<T> InternerWrap for T {
    fn wrp<'a>(&'a self, reg: &'a Interner) -> WithInterner<&'a Self> {
        WithInterner(reg, self)
    }
}

impl<'a> Debug for WithInterner<'a, &Spur> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.0.resolve(&self.1), f)
    }
}

impl<'a> Debug for WithInterner<'a, &Vec<Spur>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.1.iter().map(|s| s.wrp(self.0)))
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Option<Spur>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*&self.1.map(|s| self.0.resolve(&s)), f)
    }
}

impl<'a> Debug for WithInterner<'a, &Platform> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Platform")
            .field("name", &s.name.wrp(reg))
            .field("protect", &s.protect.wrp(reg))
            .field("comment", &s.comment)
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Tag")
            .field("name", &s.name.wrp(reg))
            .field("author", &s.author)
            .field("contact", &s.contact)
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Toplevel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_tuple("Toplevel")
            .field(&s.0.wrp(reg))
            .field(&s.1.wrp(reg))
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &CDecl> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        CDecl::fmt(&self.1, f, self.0)?;
        f.write_char('"')
    }
}

impl<'a> Debug for WithInterner<'a, &EnumValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            EnumValue::Alias(s) => f.debug_tuple("Alias").field(&s.wrp(reg)).finish(),
            EnumValue::Bitpos(b) => f.debug_tuple("Bitpos").field(b).finish(),
            EnumValue::Value(v) => f.debug_tuple("Value").field(v).finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &CommandParameter> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("CommandParameter")
            .field("name", &s.name.wrp(reg))
            .field("len", &s.len)
            .field("alt_len", &s.alt_len)
            .field("optional", &s.optional)
            .field("externsync", &s.externsync)
            .field("ty", &s.ty.wrp(reg))
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Feature> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Feature")
            .field("name", &s.name.wrp(reg))
            .field("api", &s.api.wrp(reg))
            .field("number", &s.number.wrp(reg))
            .field("protect", &s.protect.wrp(reg))
            .field(
                "children",
                &SliceDebug::new(s.children.iter().map(|c| c.wrp(reg))),
            )
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &FeatureExtensionItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match &self.1 {
            FeatureExtensionItem::Comment(arg0) => f.debug_tuple("Comment").field(arg0).finish(),
            FeatureExtensionItem::Require {
                profile,
                api,
                extension,
                feature,
                items,
            } => f
                .debug_struct("Require")
                .field("profile", &profile.wrp(reg))
                .field("api", &api.wrp(reg))
                .field("extension", &extension.wrp(reg))
                .field("feature", &feature.wrp(reg))
                .field("items", &SliceDebug::new(items.iter().map(|a| a.wrp(reg))))
                .finish(),
            FeatureExtensionItem::Remove {
                profile,
                api,
                items,
            } => f
                .debug_struct("Remove")
                .field("profile", &profile.wrp(reg))
                .field("api", &api.wrp(reg))
                .field("items", &items.wrp(reg))
                .finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &InterfaceItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match &self.1 {
            InterfaceItem::Simple { name, api } => f
                .debug_struct("Simple")
                .field("name", &name.wrp(reg))
                .field("api", &api.wrp(reg))
                .finish(),
            InterfaceItem::Extend {
                name,
                extends,
                api,
                method,
            } => f
                .debug_struct("Extend")
                .field("name", &name.wrp(reg))
                .field("extends", &extends.wrp(reg))
                .field("api", &api.wrp(reg))
                .field("method", &method.wrp(reg))
                .finish(),
            InterfaceItem::AddConstant { name, value } => f
                .debug_struct("AddConstant")
                .field("name", &name.wrp(reg))
                .field("value", &value.wrp(reg))
                .finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &ExtendMethod> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            ExtendMethod::Alias(s) => f.debug_tuple("Alias").field(&s.wrp(self.0)).finish(),
            ExtendMethod::Bitpos(b) => f.debug_tuple("Bitpos").field(b).finish(),
            ExtendMethod::BitposExtnumber { extnumber, offset } => f
                .debug_struct("BitposExtnumber")
                .field("extnumber", extnumber)
                .field("offset", offset)
                .finish(),
            ExtendMethod::Value(v) => f.debug_tuple("Value").field(v).finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &ConstantValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            ConstantValue::Alias(s) => f.debug_tuple("Alias").field(&s.wrp(self.0)).finish(),
            ConstantValue::Value(v) => f.debug_tuple("Value").field(v).finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &ToplevelBody> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            ToplevelBody::Alias(of) => f.debug_tuple("Alias").field(&of.wrp(reg)).finish(),
            ToplevelBody::Redeclaration(ty) => {
                f.debug_tuple("Redeclaration").field(&ty.wrp(reg)).finish()
            }
            ToplevelBody::Define { body } => f.debug_struct("Define").field("body", body).finish(),
            ToplevelBody::Included { header } => f
                .debug_struct("Included")
                .field("header", &header.wrp(reg))
                .finish(),
            ToplevelBody::Basetype { code } => {
                f.debug_struct("Basetype").field("code", code).finish()
            }
            ToplevelBody::Bitmask { ty, bits_enum } => f
                .debug_struct("Bitmask")
                .field("ty", &ty.wrp(reg))
                .field("bits_enum", &bits_enum.wrp(reg))
                .finish(),
            ToplevelBody::Handle {
                object_type,
                dispatchable,
            } => f
                .debug_struct("Handle")
                .field("object_type", &object_type.wrp(reg))
                .field("dispatchable", dispatchable)
                .finish(),
            ToplevelBody::Funcpointer { return_type, args } => f
                .debug_struct("Funcpointer")
                .field("return_type", &return_type.wrp(reg))
                .field(
                    "args",
                    &SliceDebug::new(args.iter().map(|(name, ty)| (name.wrp(reg), ty.wrp(reg)))),
                )
                .finish(),
            ToplevelBody::Struct { union, members } => f
                .debug_struct("Struct")
                .field("union", union)
                .field(
                    "members",
                    &SliceDebug::new(
                        members
                            .iter()
                            .map(|(name, ty)| (name.wrp(reg), ty.wrp(reg))),
                    ),
                )
                .finish(),
            ToplevelBody::Constant { ty, val } => f
                .debug_struct("Constant")
                .field("ty", &ty.wrp(reg))
                .field("val", &val.wrp(reg))
                .finish(),
            ToplevelBody::Enum { members } => f
                .debug_struct("Enum")
                .field(
                    "members",
                    &SliceDebug::new(
                        members
                            .iter()
                            .map(|(name, ty)| (name.wrp(reg), ty.wrp(reg))),
                    ),
                )
                .finish(),
            ToplevelBody::BitmaskBits { members } => f
                .debug_struct("BitmaskBits")
                .field(
                    "members",
                    &SliceDebug::new(
                        members
                            .iter()
                            .map(|(name, ty)| (name.wrp(reg), ty.wrp(reg))),
                    ),
                )
                .finish(),
            ToplevelBody::Command {
                return_type,
                params,
            } => f
                .debug_struct("Command")
                .field("return_type", &return_type.wrp(reg))
                .field(
                    "params",
                    &SliceDebug::new(params.iter().map(|p| p.wrp(reg))),
                )
                .finish(),
        }
    }
}

impl<'a> Debug for WithInterner<'a, &Extension> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Extension")
            .field("name", &s.name.wrp(reg))
            .field("number", &s.number)
            .field("sortorder", &s.sortorder)
            .field("author", &s.author)
            .field("contact", &s.contact)
            .field("ext_type", &s.ext_type.wrp(reg))
            .field("requires", &s.requires.wrp(reg))
            .field("requires_core", &s.requires_core.wrp(reg))
            .field("protect", &s.protect.wrp(reg))
            .field("platform", &s.platform.wrp(reg))
            .field("supported", &s.supported.wrp(reg))
            .field("promotedto", &s.promotedto.wrp(reg))
            .field("deprecatedby", &s.deprecatedby.wrp(reg))
            .field("obsoletedby", &s.obsoletedby.wrp(reg))
            .field("provisional", &s.provisional)
            .field("specialuse", &s.specialuse.wrp(reg))
            .field(
                "children",
                &SliceDebug::new(s.children.iter().map(|c| c.wrp(reg))),
            )
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Format> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Format")
            .field("name", &s.name.wrp(reg))
            .field("class", &s.class.wrp(reg))
            .field("blocksize", &s.blocksize)
            .field("texels_per_block", &s.texels_per_block)
            .field("block_extent", &s.block_extent)
            .field("packed", &s.packed)
            .field("compressed", &s.compressed.wrp(reg))
            .field("chroma", &s.chroma.as_ref().map(|c| c.wrp(reg)))
            .field(
                "components",
                &SliceDebug::new(s.components.iter().map(|a| a.wrp(reg))),
            )
            .field(
                "planes",
                &SliceDebug::new(s.planes.iter().map(|a| a.wrp(reg))),
            )
            .field("spirvimageformats", &s.spirvimageformats.wrp(reg))
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &YCBREncoding> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let str = match s {
            YCBREncoding::E420 => "E420",
            YCBREncoding::E422 => "E422",
            YCBREncoding::E444 => "E444",
        };
        f.write_str(str)
    }
}

impl<'a> Debug for WithInterner<'a, &NumericFormat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let str = match s {
            NumericFormat::SFLOAT => "SFLOAT",
            NumericFormat::SINT => "SINT",
            NumericFormat::SNORM => "SNORM",
            NumericFormat::SRGB => "SRGB",
            NumericFormat::SSCALED => "SSCALED",
            NumericFormat::UFLOAT => "UFLOAT",
            NumericFormat::UINT => "UINT",
            NumericFormat::UNORM => "UNORM",
            NumericFormat::USCALED => "USCALED",
        };
        f.write_str(str)
    }
}

impl<'a> Debug for WithInterner<'a, &Component> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Component")
            .field("name", &s.name.wrp(reg))
            .field("bits", &s.bits)
            .field("numeric_format", &s.numeric_format.wrp(reg))
            .field("plane_index", &s.plane_index)
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &Plane> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Plane")
            .field("index", &s.index)
            .field("width_divisor", &s.width_divisor)
            .field("height_divisor", &s.height_divisor)
            .field("compatible", &s.compatible.wrp(reg))
            .finish()
    }
}

impl<'a> Debug for WithInterner<'a, &SpirvExtCap> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_tuple("SpirvExtCap")
            .field(&s.0.wrp(reg))
            .field(&SliceDebug::new(s.1.iter().map(|a| a.wrp(reg))))
            .finish()
    }
}
impl<'a> Debug for WithInterner<'a, &SpirvEnable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            SpirvEnable::Version(s) => f.debug_tuple("Version").field(&s.wrp(reg)).finish(),
            SpirvEnable::Extension(s) => f.debug_tuple("Extension").field(&s.wrp(reg)).finish(),
            SpirvEnable::Feature {
                structure,
                feature,
                requires,
                alias,
            } => f
                .debug_struct("Feature")
                .field("structure", &structure.wrp(reg))
                .field("feature", &feature.wrp(reg))
                .field("requires", &requires.wrp(reg))
                .field("alias", &alias.wrp(reg))
                .finish(),
            SpirvEnable::Property {
                property,
                member,
                value,
                requires,
            } => f
                .debug_struct("Property")
                .field("property", &property.wrp(reg))
                .field("member", &member.wrp(reg))
                .field("value", &value.wrp(reg))
                .field("requires", &requires.wrp(reg))
                .finish(),
        }
    }
}

impl Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self;

        let platforms = &SliceDebug::new(self.platforms.iter().map(|a| a.wrp(reg)));
        let tags = &SliceDebug::new(self.tags.iter().map(|a| a.wrp(reg)));
        let headers = &SliceDebug::new(self.headers.iter().map(|a| a.wrp(reg)));
        let toplevel = &SliceDebug::new(self.toplevel.iter().map(|a| a.wrp(reg)));
        let features = &SliceDebug::new(self.features.iter().map(|a| a.wrp(reg)));
        let extensions = &SliceDebug::new(self.extensions.iter().map(|a| a.wrp(reg)));
        let formats = &SliceDebug::new(self.formats.iter().map(|a| a.wrp(reg)));
        let spirv_capabilities =
            &SliceDebug::new(self.spirv_capabilities.iter().map(|a| a.wrp(reg)));
        let spirv_extensions = &SliceDebug::new(self.spirv_extensions.iter().map(|a| a.wrp(reg)));

        f.debug_struct("Registry")
            .field("platforms", platforms)
            .field("tags", tags)
            .field("headers", headers)
            .field("toplevel", toplevel)
            .field("features", features)
            .field("extensions", extensions)
            .field("formats", formats)
            .field("spirv_capabilities", spirv_capabilities)
            .field("spirv_extensions", spirv_extensions)
            // .field("item_map", item_map)
            // .field("interner", interner)
            .finish()
    }
}
