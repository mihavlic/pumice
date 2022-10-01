use generator_lib::{
    interner::UniqueStr,
    type_declaration::{TyToken, Type, TypeRef},
    Declaration, RedeclarationMethod, Symbol, SymbolBody,
};

use crate::{context::Context, switch};

use super::type_analysis::is_void_pointer;

pub struct DeriveData {
    eq: Vec<Option<bool>>,
    default: Vec<Option<bool>>,
    force_copy: Vec<bool>,
    value: Vec<Option<bool>>,
    no_void: Vec<Option<bool>>,
}

impl DeriveData {
    pub fn new(ctx: &Context) -> Self {
        let mut symbol_force_copy = vec![false; ctx.reg.symbols.len()];

        // due to rust trying to somehow make unions make safe, there is an uncertainty on how
        // destructors should work with them, currently only Copy types can be in them, thus we
        // declare all their constituents Copy
        // TODO perhaps it's better to manually implement Copy only for the direct constituents?
        for Symbol(name, body) in &ctx.reg.symbols {
            match body {
                &SymbolBody::Struct { union, .. } => {
                    if union {
                        broadcast_to_constituents(*name, &mut symbol_force_copy, true, &ctx)
                    }
                }
                _ => {}
            }
        }

        Self {
            eq: vec![None; ctx.reg.symbols.len()],
            default: vec![None; ctx.reg.symbols.len()],
            force_copy: symbol_force_copy,
            value: vec![None; ctx.reg.symbols.len()],
            no_void: vec![None; ctx.reg.symbols.len()],
        }
    }
    pub fn is_copy(&self, name: UniqueStr, ctx: &Context) -> bool {
        let index = ctx.get_symbol_index(name).unwrap() as usize;
        self.force_copy[index]
    }
    pub fn is_eq(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_hashable(name, &mut self.eq, ctx)
    }
    pub fn is_zeroable(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_zeroable(name, &mut self.default, ctx)
    }
    pub fn is_value_type(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_value(name, &mut self.value, ctx)
    }
    /// This means that the type doesn't contain any void pointers that are not pNext or don't have an associated size
    pub fn is_no_void(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_no_void(name, &mut self.no_void, ctx)
    }
    pub fn type_is_eq(&mut self, ty: &TypeRef, ctx: &Context) -> bool {
        type_is_hashable(ty, &mut self.eq, ctx)
    }
    pub fn type_is_zeroable(&mut self, ty: &TypeRef, ctx: &Context) -> bool {
        type_is_zeroable(ty, &mut self.default, ctx)
    }
    pub fn type_is_value(&mut self, ty: &TypeRef, ctx: &Context) -> bool {
        type_is_value(ty, &mut self.value, ctx)
    }
    /// This means that the type doesn't contain any void pointers that are not pNext or don't have an associated size
    pub fn type_is_no_void(&mut self, ty: &TypeRef, ctx: &Context) -> bool {
        type_is_no_void(ty, &mut self.no_void, ctx)
    }
}

pub fn broadcast_to_constituents(
    name: UniqueStr,
    symbol_overlay: &mut [bool],
    to: bool,
    ctx: &Context,
) {
    let index = ctx.get_symbol_index(name).unwrap() as usize;

    let cell = &mut symbol_overlay[index];

    if *cell == to {
        return;
    }

    *cell = to;

    match &ctx.reg.symbols[index].1 {
        SymbolBody::Alias(of) => broadcast_to_constituents(*of, symbol_overlay, to, ctx),
        SymbolBody::Redeclaration(method) => match method {
            RedeclarationMethod::Type(ty) => {
                broadcast_to_constituents(ty.get_basetype(), symbol_overlay, to, ctx)
            }
            // as of now only used for functions that take primitive types
            RedeclarationMethod::Custom(_) => {}
        },
        SymbolBody::Enum { ty, .. } => broadcast_to_constituents(*ty, symbol_overlay, to, ctx),
        SymbolBody::Struct { members, .. } => {
            for member in members {
                broadcast_to_constituents(member.ty.get_basetype(), symbol_overlay, to, ctx)
            }
        }
        SymbolBody::Basetype { .. }
        | SymbolBody::Handle { .. }
        | SymbolBody::Funcpointer { .. } => {}
        SymbolBody::Constant { .. }
        | SymbolBody::Command { .. }
        | SymbolBody::Included { .. }
        | SymbolBody::Define { .. } => {
            unreachable!()
        }
    };
}

pub fn logical_and_constituents<
    F1: Fn(&Symbol) -> bool,
    F2: Fn(&Type, &mut [Option<bool>]) -> bool,
    F3: Fn(&Declaration, &mut [Option<bool>]) -> bool,
>(
    name: UniqueStr,
    on_leaf: F1,
    on_type: F2,
    on_decl: F3,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    debug_assert!(symbol_overlay.len() >= ctx.reg.symbols.len());

    let index = ctx.get_symbol_index(name).unwrap() as usize;

    if let Some(computed) = symbol_overlay[index] {
        return computed;
    }

    let symbol = &ctx.reg.symbols[index];
    let fresh = match &symbol.1 {
        SymbolBody::Alias(of) => {
            logical_and_constituents(*of, on_leaf, on_type, on_decl, symbol_overlay, ctx)
        }
        SymbolBody::Redeclaration(method) => match method {
            RedeclarationMethod::Type(ty) => on_type(ty, symbol_overlay),
            RedeclarationMethod::Custom(_) => unreachable!(),
        },
        SymbolBody::Struct { members, .. } => {
            let early = on_leaf(symbol);
            if early == true {
                members.iter().all(|m| on_decl(m, symbol_overlay))
            } else {
                false
            }
        }
        SymbolBody::Basetype { .. }
        | SymbolBody::Enum { .. }
        | SymbolBody::Handle { .. }
        | SymbolBody::Funcpointer { .. } => on_leaf(symbol),
        SymbolBody::Constant { .. }
        | SymbolBody::Command { .. }
        | SymbolBody::Included { .. }
        | SymbolBody::Define { .. } => {
            unreachable!()
        }
    };

    symbol_overlay[index] = Some(fresh);
    fresh
}

pub fn symbol_is_hashable(
    name: UniqueStr,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    logical_and_constituents(
        name,
        |body| match &body.1 {
            // unions are not hashable
            SymbolBody::Basetype { .. } => {
                let s = &ctx.strings;
                switch!(body.0;
                    s.void, s.float, s.double => false;
                    @ true
                )
            }
            SymbolBody::Struct { union, .. } => *union == false,
            SymbolBody::Enum { .. } => true,
            SymbolBody::Handle { .. } => true,
            // a function pointer lives forever and is unique, right?
            SymbolBody::Funcpointer { .. } => true,
            _ => unreachable!(),
        },
        |ty, overlay| type_is_hashable(ty, overlay, ctx),
        |decl, overlay| type_is_hashable(&decl.ty, overlay, ctx),
        symbol_overlay,
        ctx,
    )
}

pub fn type_is_hashable(ty: &TypeRef, symbol_overlay: &mut [Option<bool>], ctx: &Context) -> bool {
    if let Some(basetype) = ty.try_only_basetype() {
        return symbol_is_hashable(basetype, symbol_overlay, ctx);
    } else {
        for token in ty.as_slice() {
            match token {
                // TODO reconsider this
                // hashing is really only applicable where we can own the key and we can't really ensure that with pointers
                TyToken::Ptr => return false,
                TyToken::BaseType(b) => {
                    return type_is_hashable(&Type::from_only_basetype(*b), symbol_overlay, ctx)
                }
                _ => {}
            }
        }

        unreachable!()
    }
}

pub fn symbol_is_zeroable(
    name: UniqueStr,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    logical_and_constituents(
        name,
        |body| match &body.1 {
            SymbolBody::Basetype { .. } => true,
            SymbolBody::Struct { .. } => true,
            SymbolBody::Enum { .. } => true,
            SymbolBody::Handle { .. } => true,
            SymbolBody::Funcpointer { .. } => false,
            _ => unreachable!(),
        },
        |ty, overlay| type_is_zeroable(ty, overlay, ctx),
        |decl, overlay| type_is_zeroable(&decl.ty, overlay, ctx),
        symbol_overlay,
        ctx,
    )
}

fn type_is_zeroable(ty: &TypeRef, overlay: &mut [Option<bool>], ctx: &Context) -> bool {
    if let Some(ty) = ty.try_only_basetype() {
        symbol_is_zeroable(ty, overlay, ctx)
    } else {
        true
    }
}

pub fn symbol_is_value(
    name: UniqueStr,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    logical_and_constituents(
        name,
        |body| match &body.1 {
            SymbolBody::Basetype { .. } => true,
            SymbolBody::Struct { .. } => true,
            SymbolBody::Enum { .. } => true,
            SymbolBody::Handle { .. } => true,
            SymbolBody::Funcpointer { .. } => true,
            _ => unreachable!(),
        },
        |ty, overlay| type_is_value(ty, overlay, ctx),
        |decl, overlay| type_is_value(&decl.ty, overlay, ctx),
        symbol_overlay,
        ctx,
    )
}

pub fn type_is_value(ty: &TypeRef, symbol_overlay: &mut [Option<bool>], ctx: &Context) -> bool {
    if let Some(basetype) = ty.try_only_basetype() {
        return symbol_is_value(basetype, symbol_overlay, ctx);
    } else {
        for token in ty.as_slice() {
            match token {
                TyToken::Ptr | TyToken::Ref => return false,
                TyToken::BaseType(b) => {
                    return type_is_value(&Type::from_only_basetype(*b), symbol_overlay, ctx)
                }
                _ => {}
            }
        }

        unreachable!()
    }
}

pub fn symbol_is_no_void(
    name: UniqueStr,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    logical_and_constituents(
        name,
        |body| match &body.1 {
            SymbolBody::Struct { .. } => true,
            SymbolBody::Enum { .. } => true,
            SymbolBody::Handle { .. } => true,
            SymbolBody::Funcpointer { .. } => true,
            _ => unreachable!(),
        },
        |ty, overlay| type_is_no_void(ty, overlay, ctx),
        |decl, overlay| {
            if decl.name == ctx.strings.pNext || !decl.metadata.length.is_empty() {
                return true;
            }
            type_is_no_void(&*decl.ty, overlay, ctx)
        },
        symbol_overlay,
        ctx,
    )
}

fn type_is_no_void(ty: &TypeRef, overlay: &mut [Option<bool>], ctx: &Context) -> bool {
    !is_void_pointer(ty, ctx)
        && ty
            .try_not_only_basetype()
            .map(|b| symbol_is_no_void(b, overlay, ctx))
            .unwrap_or(true)
}
