use smallvec::SmallVec;

use crate::{
    interner::UniqueStr, type_declaration::Type, ConstantValue, Declaration, DeclarationMetadata,
    Extension, Feature, FeatureExtensionItem, InterfaceItem, RedeclarationMethod, Symbol,
    SymbolBody,
};

pub trait ForeachUniquestr {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F);
}

impl<T: ForeachUniquestr> ForeachUniquestr for Vec<T> {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        for item in self {
            item.foreach(fun)
        }
    }
}

impl<T: ForeachUniquestr, A: smallvec::Array<Item = T>> ForeachUniquestr for SmallVec<A> {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        for item in self {
            item.foreach(fun)
        }
    }
}

impl<T: ForeachUniquestr> ForeachUniquestr for Option<T> {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        if let Some(heretic) = self.as_mut() {
            heretic.foreach(fun);
        }
    }
}

impl<T1: ForeachUniquestr, T2: ForeachUniquestr> ForeachUniquestr for (T1, T2) {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.0.foreach(fun);
        self.1.foreach(fun);
    }
}

impl ForeachUniquestr for UniqueStr {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        fun(self);
    }
}

impl ForeachUniquestr for Type {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.get_basetype_mut().foreach(fun)
    }
}

impl ForeachUniquestr for RedeclarationMethod {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        match self {
            RedeclarationMethod::Type(ty) => ty.foreach(fun),
            RedeclarationMethod::Custom(_) => {}
        }
    }
}

impl ForeachUniquestr for ConstantValue {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        match self {
            ConstantValue::Symbol(s) => s.foreach(fun),
            _ => {}
        }
    }
}

impl ForeachUniquestr for DeclarationMetadata {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.values.foreach(fun);
        self.length.foreach(fun);
    }
}

impl ForeachUniquestr for Declaration {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.name.foreach(fun);
        self.ty.foreach(fun);
        self.metadata.foreach(fun);
    }
}

impl ForeachUniquestr for SymbolBody {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        match self {
            SymbolBody::Alias(of) => of.foreach(fun),
            SymbolBody::Redeclaration(method) => method.foreach(fun),
            SymbolBody::Included { .. } => {}
            SymbolBody::Define { .. } => {}
            SymbolBody::Basetype { .. } => {}
            SymbolBody::Enum { ty, members, .. } => {
                ty.foreach(fun);
                members.foreach(fun);
            }
            SymbolBody::Handle { object_type, .. } => object_type.foreach(fun),
            SymbolBody::Funcpointer { ret, args } => {
                ret.foreach(fun);
                args.foreach(fun);
            }
            SymbolBody::Struct { members, .. } => members.foreach(fun),
            SymbolBody::Constant { val, ty } => {
                val.foreach(fun);
                ty.foreach(fun);
            }
            SymbolBody::Command {
                success_codes,
                error_codes,
                return_type,
                params,
            } => {
                success_codes.foreach(fun);
                error_codes.foreach(fun);
                return_type.foreach(fun);
                params.foreach(fun);
            }
        }
    }
}

impl ForeachUniquestr for Symbol {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.0.foreach(fun);
        self.1.foreach(fun);
    }
}

impl ForeachUniquestr for InterfaceItem {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        match self {
            InterfaceItem::Simple { name } => name.foreach(fun),
            InterfaceItem::Extend {
                name,
                extends,
                value,
            } => {
                name.foreach(fun);
                extends.foreach(fun);
                value.foreach(fun);
            }
        }
    }
}

impl ForeachUniquestr for FeatureExtensionItem {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        match self {
            FeatureExtensionItem::Comment(_) => {}
            FeatureExtensionItem::Require {
                profile,
                api,
                extension,
                feature,
                items,
            } => {
                profile.foreach(fun);
                api.foreach(fun);
                extension.foreach(fun);
                feature.foreach(fun);
                items.foreach(fun);
            }
            FeatureExtensionItem::Remove {
                profile,
                api,
                items,
            } => {
                profile.foreach(fun);
                api.foreach(fun);
                items.foreach(fun);
            }
        }
    }
}

impl ForeachUniquestr for Feature {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.name.foreach(fun);
        self.api.foreach(fun);
        self.protect.foreach(fun);
        self.children.foreach(fun);
    }
}

impl ForeachUniquestr for Extension {
    fn foreach<F: FnMut(&mut UniqueStr)>(&mut self, fun: &mut F) {
        self.name.foreach(fun);
        self.requires.foreach(fun);
        self.requires_core.foreach(fun);
        self.protect.foreach(fun);
        self.platform.foreach(fun);
        self.supported.foreach(fun);
        self.promotedto.foreach(fun);
        self.deprecatedby.foreach(fun);
        self.obsoletedby.foreach(fun);
        self.specialuse.foreach(fun);
        self.children.foreach(fun);
    }
}
