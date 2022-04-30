use std::fmt::Debug;

use lasso::{Rodeo, Spur};

use crate::{format_utils::RegistryDisplay, Define, Registry, Toplevel, ToplevelBody};

// duplicated from format_utils because we want a slightly different behaviour for
// some types such as emitting "" for strings
struct WithRodeo<'a, T: ?Sized>(&'a Rodeo, T);

trait RodeoWrap {
    fn reg<'a>(&'a self, reg: &'a Rodeo) -> WithRodeo<&'a Self>;
}

impl<T> RodeoWrap for T {
    fn reg<'a>(&'a self, reg: &'a Rodeo) -> WithRodeo<&'a Self> {
        WithRodeo(reg, self)
    }
}

impl<'a> Debug for WithRodeo<'a, &Spur> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self.0.resolve(&self.1), f)
    }
}

impl<'a> Debug for WithRodeo<'a, &Option<Spur>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*&self.1.map(|s| self.0.resolve(&s)), f)
    }
}

impl<'a> Debug for WithRodeo<'a, &Define> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_struct("Define")
            .field("name", &s.name.reg(reg))
            .field("body", &s.body)
            .finish()
    }
}

impl<'a> Debug for WithRodeo<'a, &Toplevel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.1;
        let reg = self.0;
        f.debug_tuple("Toplevel")
            .field(&s.0.reg(reg))
            .field(&s.1.reg(reg))
            .finish()
    }
}

impl<'a> Debug for WithRodeo<'a, &ToplevelBody> {
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
                .field("return_type", return_type)
                .field("args", args)
                .finish(),
            ToplevelBody::Struct { union, members } => f
                .debug_struct("Struct")
                .field("union", union)
                .field("members", members)
                .finish(),
            ToplevelBody::Constant { ty, val } => f
                .debug_struct("Constant")
                .field("ty", &ty.reg(reg))
                .field("val", &val.reg(reg))
                .finish(),
            ToplevelBody::Enum { members } => {
                f.debug_struct("Enum").field("members", members).finish()
            }
            ToplevelBody::BitmaskBits { members } => f
                .debug_struct("BitmaskBits")
                .field("members", members)
                .finish(),
            ToplevelBody::Command {
                return_type,
                params,
            } => f
                .debug_struct("Command")
                .field("return_type", &return_type.reg(reg))
                .field("params", params)
                .finish(),
        }
    }
}

impl Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg = &*self.interner.borrow();

        let vendors = &self.vendors.iter().flat_map(|c| &c.children);
        let platforms = &self.platforms.iter().flat_map(|c| &c.children);
        let tags = &self.tags.iter().flat_map(|c| &c.children);
        let headers = &self.headers.iter().map(|s| reg.resolve(s));
        let defines = &self.defines.iter().map(|d| d.reg(reg));
        let toplevel = &self.toplevel;
        let features = &self.features;
        let extensions = &self.extensions;
        let formats = &self.formats;
        let spirv_capabilities = &self.spirv_capabilities;
        let spirv_extensions = &self.spirv_extensions;
        let item_map = &self.item_map;

        f.debug_struct("Registry")
            .field("vendors", vendors)
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
            .finish_non_exhaustive()
    }
}
