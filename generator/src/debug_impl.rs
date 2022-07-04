use std::{
    cell::Cell,
    fmt::{Debug, Write},
};

use lasso::Spur;

use crate::{
    type_declaration::TypeDecl, CommandParameter, Component, ConstantValue, Define, EnumValue,
    ExtendMethod, Extension, Feature, FeatureExtensionItem, Format, InterfaceItem, Plane, Platform,
    Registry, SpirvEnable, SpirvExtCap, Tag, Toplevel, ToplevelBody,
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
struct WithRegistry<'a, T: ?Sized>(&'a Registry, T);

trait RodeoWrap {
    fn reg<'a>(&'a self, reg: &'a Registry) -> WithRegistry<&'a Self>;
}

impl<T> RodeoWrap for T {
    fn reg<'a>(&'a self, reg: &'a Registry) -> WithRegistry<&'a Self> {
        WithRegistry(reg, self)
    }
}

impl<'a> Debug for WithRegistry<'a, &Spur> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.0.resolve(&self.1), f)
    }
}

impl<'a> Debug for WithRegistry<'a, &Vec<Spur>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.1.iter().map(|s| s.reg(&self.0)))
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Option<Spur>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*&self.1.map(|s| self.0.resolve(&s)), f)
    }
}

impl<'a> Debug for WithRegistry<'a, &Platform> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Platform")
            .field("name", &s.name.reg(reg))
            .field("protect", &s.protect.reg(reg))
            .field("comment", &s.comment)
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Tag")
            .field("name", &s.name.reg(reg))
            .field("author", &s.author)
            .field("contact", &s.contact)
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Define> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Define")
            .field("name", &s.name.reg(reg))
            .field("body", &s.body)
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Toplevel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_tuple("Toplevel")
            .field(&s.0.reg(reg))
            .field(&s.1.reg(reg))
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &TypeDecl> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        TypeDecl::fmt(&self.1, f, self.0)?;
        f.write_char('"')
    }
}

impl<'a> Debug for WithRegistry<'a, &EnumValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            EnumValue::Alias(s) => f.debug_tuple("Alias").field(&s.reg(reg)).finish(),
            other => other.fmt(f),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &CommandParameter> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("CommandParameter")
            .field("name", &s.name.reg(reg))
            .field("len", &s.len)
            .field("alt_len", &s.alt_len)
            .field("optional", &s.optional)
            .field("externsync", &s.externsync)
            .field("ty", &s.ty.reg(reg))
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Feature> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Feature")
            .field("name", &s.name.reg(reg))
            .field("api", &s.api.reg(reg))
            .field("number", &s.number.reg(reg))
            .field("protect", &s.protect.reg(reg))
            .field(
                "children",
                &SliceDebug::new(s.children.iter().map(|c| c.reg(reg))),
            )
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &FeatureExtensionItem> {
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
                .field("profile", &profile.reg(reg))
                .field("api", &api.reg(reg))
                .field("extension", &extension.reg(reg))
                .field("feature", &feature.reg(reg))
                .field("items", &SliceDebug::new(items.iter().map(|a| a.reg(reg))))
                .finish(),
            FeatureExtensionItem::Remove {
                profile,
                api,
                items,
            } => f
                .debug_struct("Remove")
                .field("profile", &profile.reg(reg))
                .field("api", &api.reg(reg))
                .field("items", &items.reg(reg))
                .finish(),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &InterfaceItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match &self.1 {
            InterfaceItem::Simple { name, api } => f
                .debug_struct("Simple")
                .field("name", &name.reg(reg))
                .field("api", &api.reg(reg))
                .finish(),
            InterfaceItem::Extend {
                name,
                extends,
                api,
                method,
            } => f
                .debug_struct("Extend")
                .field("name", &name.reg(reg))
                .field("extends", &extends.reg(reg))
                .field("api", &api.reg(reg))
                .field("method", &method.reg(reg))
                .finish(),
            InterfaceItem::AddConstant { name, value } => f
                .debug_struct("AddConstant")
                .field("name", &name.reg(reg))
                .field("value", &value.reg(reg))
                .finish(),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &ExtendMethod> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            ExtendMethod::Alias(s) => f.debug_tuple("Alias").field(&s.reg(self.0)).finish(),
            other => other.fmt(f),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &ConstantValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            ConstantValue::Alias(s) => f.debug_tuple("Alias").field(&s.reg(self.0)).finish(),
            other => other.fmt(f),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &ToplevelBody> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            ToplevelBody::Alias { alias_of, kind } => f
                .debug_struct("Alias")
                .field("alias_of", &alias_of.reg(reg))
                .field("kind", kind)
                .finish(),
            ToplevelBody::Included { header } => f
                .debug_struct("Included")
                .field("header", &header.reg(reg))
                .finish(),
            ToplevelBody::Basetype { ty, code } => f
                .debug_struct("Basetype")
                .field("ty", &ty.reg(reg))
                .field("code", code)
                .finish(),
            ToplevelBody::Bitmask { ty, bits_enum } => f
                .debug_struct("Bitmask")
                .field("ty", &ty.reg(reg))
                .field("bits_enum", &bits_enum.reg(reg))
                .finish(),
            ToplevelBody::Handle {
                object_type,
                dispatchable,
            } => f
                .debug_struct("Handle")
                .field("object_type", &object_type.reg(reg))
                .field("dispatchable", dispatchable)
                .finish(),
            ToplevelBody::Funcpointer { return_type, args } => f
                .debug_struct("Funcpointer")
                .field("return_type", &return_type.reg(reg))
                .field(
                    "args",
                    &SliceDebug::new(args.iter().map(|(name, ty)| (name.reg(reg), ty.reg(reg)))),
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
                            .map(|(name, ty)| (name.reg(reg), ty.reg(reg))),
                    ),
                )
                .finish(),
            ToplevelBody::Constant { ty, val } => f
                .debug_struct("Constant")
                .field("ty", &ty.reg(reg))
                .field("val", &val.reg(reg))
                .finish(),
            ToplevelBody::Enum { members } => f
                .debug_struct("Enum")
                .field(
                    "members",
                    &SliceDebug::new(
                        members
                            .iter()
                            .map(|(name, ty)| (name.reg(reg), ty.reg(reg))),
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
                            .map(|(name, ty)| (name.reg(reg), ty.reg(reg))),
                    ),
                )
                .finish(),
            ToplevelBody::Command {
                return_type,
                params,
            } => f
                .debug_struct("Command")
                .field("return_type", &return_type.reg(reg))
                .field(
                    "params",
                    &SliceDebug::new(params.iter().map(|p| p.reg(reg))),
                )
                .finish(),
        }
    }
}

impl<'a> Debug for WithRegistry<'a, &Extension> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Extension")
            .field("name", &s.name.reg(reg))
            .field("number", &s.number)
            .field("sortorder", &s.sortorder)
            .field("author", &s.author)
            .field("contact", &s.contact)
            .field("ext_type", &s.ext_type.reg(reg))
            .field("requires", &s.requires.reg(reg))
            .field("requires_core", &s.requires_core.reg(reg))
            .field("protect", &s.protect.reg(reg))
            .field("platform", &s.platform.reg(reg))
            .field("supported", &s.supported.reg(reg))
            .field("promotedto", &s.promotedto.reg(reg))
            .field("deprecatedby", &s.deprecatedby.reg(reg))
            .field("obsoletedby", &s.obsoletedby.reg(reg))
            .field("provisional", &s.provisional)
            .field("specialuse", &s.specialuse.reg(reg))
            .field(
                "children",
                &SliceDebug::new(s.children.iter().map(|c| c.reg(reg))),
            )
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Format> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Format")
            .field("name", &s.name.reg(reg))
            .field("class", &s.class.reg(reg))
            .field("blocksize", &s.blocksize)
            .field("texels_per_block", &s.texels_per_block)
            .field("block_extent", &s.block_extent)
            .field("packed", &s.packed)
            .field("compressed", &s.compressed.reg(reg))
            .field("chroma", &s.chroma)
            .field(
                "components",
                &SliceDebug::new(s.components.iter().map(|a| a.reg(reg))),
            )
            .field(
                "planes",
                &SliceDebug::new(s.planes.iter().map(|a| a.reg(reg))),
            )
            .field("spirvimageformats", &s.spirvimageformats.reg(reg))
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Component> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Component")
            .field("name", &s.name.reg(reg))
            .field("bits", &s.bits)
            .field("numeric_format", &s.numeric_format)
            .field("plane_index", &s.plane_index)
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &Plane> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_struct("Plane")
            .field("index", &s.index)
            .field("width_divisor", &s.width_divisor)
            .field("height_divisor", &s.height_divisor)
            .field("compatible", &s.compatible.reg(reg))
            .finish()
    }
}

impl<'a> Debug for WithRegistry<'a, &SpirvExtCap> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        let s = self.1;
        f.debug_tuple("SpirvExtCap")
            .field(&s.0.reg(reg))
            .field(&SliceDebug::new(s.1.iter().map(|a| a.reg(reg))))
            .finish()
    }
}
impl<'a> Debug for WithRegistry<'a, &SpirvEnable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self.0;
        match self.1 {
            SpirvEnable::Version(s) => f.debug_tuple("Version").field(&s.reg(reg)).finish(),
            SpirvEnable::Extension(s) => f.debug_tuple("Extension").field(&s.reg(reg)).finish(),
            SpirvEnable::Feature {
                structure,
                feature,
                requires,
                alias,
            } => f
                .debug_struct("Feature")
                .field("structure", &structure.reg(reg))
                .field("feature", &feature.reg(reg))
                .field("requires", &requires.reg(reg))
                .field("alias", &alias.reg(reg))
                .finish(),
            SpirvEnable::Property {
                property,
                member,
                value,
                requires,
            } => f
                .debug_struct("Property")
                .field("property", &property.reg(reg))
                .field("member", &member.reg(reg))
                .field("value", &value.reg(reg))
                .field("requires", &requires.reg(reg))
                .finish(),
        }
    }
}

impl Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = self;

        let platforms = &SliceDebug::new(self.platforms.iter().map(|a| a.reg(reg)));
        let tags = &SliceDebug::new(self.tags.iter().map(|a| a.reg(reg)));
        let headers = &SliceDebug::new(self.headers.iter().map(|a| a.reg(reg)));
        let defines = &SliceDebug::new(self.defines.iter().map(|a| a.reg(reg)));
        let toplevel = &SliceDebug::new(self.toplevel.iter().map(|a| a.reg(reg)));
        let features = &SliceDebug::new(self.features.iter().map(|a| a.reg(reg)));
        let extensions = &SliceDebug::new(self.extensions.iter().map(|a| a.reg(reg)));
        let formats = &SliceDebug::new(self.formats.iter().map(|a| a.reg(reg)));
        let spirv_capabilities =
            &SliceDebug::new(self.spirv_capabilities.iter().map(|a| a.reg(reg)));
        let spirv_extensions = &SliceDebug::new(self.spirv_extensions.iter().map(|a| a.reg(reg)));

        f.debug_struct("Registry")
            .field("platforms", platforms)
            .field("tags", tags)
            .field("headers", headers)
            .field("defines", defines)
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
