use generator_lib::{
    interner::UniqueStr,
    type_declaration::{TyToken, Type},
    Symbol, SymbolBody,
};

use crate::{is_std_type, Context};

use super::switch;

pub struct DeriveData {
    eq: Vec<Option<bool>>,
    default: Vec<Option<bool>>,
    force_copy: Vec<bool>,
}

impl DeriveData {
    pub fn new(ctx: &Context) -> Self {
        let symbol_eq = vec![None; ctx.reg.symbols.len()];
        let symbol_default = vec![None; ctx.reg.symbols.len()];
        let mut symbol_force_copy = vec![false; ctx.reg.symbols.len()];

        // due to rust trying to somehow make unions make safe, there is an uncertainty on how
        // destructors should work with them, currently only Copy types can be in them, thus we declare
        // all their constituents Copy
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
            eq: symbol_eq,
            default: symbol_default,
            force_copy: symbol_force_copy,
        }
    }
    pub fn is_copy<'a>(&self, name: UniqueStr, ctx: &Context) -> bool {
        let index = ctx.get_symbol_index(name).unwrap() as usize;
        self.force_copy[index]
    }
    pub fn is_eq<'a>(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_hashable(name, &mut self.eq, ctx)
    }
    pub fn is_default<'a>(&mut self, name: UniqueStr, ctx: &Context) -> bool {
        symbol_is_default(name, &mut self.default, ctx)
    }
}

pub fn broadcast_to_constituents(
    name: UniqueStr,
    symbol_overlay: &mut [bool],
    to: bool,
    ctx: &Context,
) {
    if is_std_type(name, ctx) {
        return;
    }

    let index = ctx.get_symbol_index(name).unwrap() as usize;

    let cell = &mut symbol_overlay[index];

    if *cell == to {
        return;
    }

    *cell = to;

    match &ctx.symbols[index].1 {
        SymbolBody::Alias(of) => broadcast_to_constituents(*of, symbol_overlay, to, ctx),
        SymbolBody::Redeclaration(ty) => {
            broadcast_to_constituents(ty.get_basetype(), symbol_overlay, to, ctx)
        }
        SymbolBody::Enum { ty, .. } => broadcast_to_constituents(*ty, symbol_overlay, to, ctx),
        SymbolBody::Struct { members, .. } => {
            for member in members {
                broadcast_to_constituents(member.ty.get_basetype(), symbol_overlay, to, ctx)
            }
        }
        SymbolBody::Handle { .. } | SymbolBody::Funcpointer { .. } => {}
        SymbolBody::Constant { .. }
        | SymbolBody::Command { .. }
        | SymbolBody::Included { .. }
        | SymbolBody::Define { .. }
        | SymbolBody::Basetype { .. } => {
            unreachable!()
        }
    };
}

pub fn logical_and_constituents<
    F1: Fn(&Symbol) -> bool,
    F3: Fn(&Type, &mut [Option<bool>]) -> bool,
>(
    name: UniqueStr,
    on_leaf: F1,
    on_type: F3,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    debug_assert!(symbol_overlay.len() >= ctx.symbols.len());

    let index = if let Some(index) = ctx.get_symbol_index(name) {
        index as usize
    } else {
        return on_type(&Type::from_only_basetype(name), symbol_overlay);
    };

    if let Some(computed) = symbol_overlay[index] {
        return computed;
    }

    let symbol = &ctx.symbols[index];
    let fresh = match &symbol.1 {
        SymbolBody::Alias(of) => {
            logical_and_constituents(*of, on_leaf, on_type, symbol_overlay, ctx)
        }
        SymbolBody::Redeclaration(ty) => on_type(ty, symbol_overlay),
        SymbolBody::Struct { members, .. } => {
            let early = on_leaf(symbol);
            if early == true {
                members.iter().all(|m| on_type(&m.ty, symbol_overlay))
            } else {
                false
            }
        }
        SymbolBody::Enum { .. } | SymbolBody::Handle { .. } | SymbolBody::Funcpointer { .. } => {
            on_leaf(symbol)
        }
        SymbolBody::Constant { .. }
        | SymbolBody::Command { .. }
        | SymbolBody::Included { .. }
        | SymbolBody::Define { .. }
        | SymbolBody::Basetype { .. } => {
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
            SymbolBody::Struct { union, .. } => *union == false,
            SymbolBody::Enum { .. } => true,
            SymbolBody::Handle { .. } => true,
            // a function pointer lives forever and is unique, right?
            SymbolBody::Funcpointer { .. } => true,
            _ => unreachable!(),
        },
        |ty, overlay| type_is_hashable(ty, overlay, ctx),
        symbol_overlay,
        ctx,
    )
}

pub fn type_is_hashable(ty: &Type, symbol_overlay: &mut [Option<bool>], ctx: &Context) -> bool {
    if let Some(basetype) = ty.try_only_basetype() {
        let s = &ctx.strings;
        switch!(basetype;
            s.void, s.float, s.double => return false;
            s.int, s.char, s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => return true;
            @ {
                assert!(ctx.get_symbol_index(basetype).is_some(), "Only a symbol should pass here.");
                return symbol_is_hashable(basetype, symbol_overlay, ctx);
            }
        );
    } else {
        for token in &ty.0 {
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

pub fn symbol_is_default(
    name: UniqueStr,
    symbol_overlay: &mut [Option<bool>],
    ctx: &Context,
) -> bool {
    logical_and_constituents(
        name,
        |body| match &body.1 {
            // should unions be default by being zeroed?
            SymbolBody::Struct { union, .. } => *union == false,
            SymbolBody::Enum { members, .. } => members
                .iter()
                .any(|(name, _)| name.resolve().contains("UNDEFINED")),
            SymbolBody::Handle { dispatchable, .. } => *dispatchable == false,
            SymbolBody::Funcpointer { .. } => true,
            _ => unreachable!(),
        },
        |ty, overlay| {
            if let Some(ty) = ty.try_only_basetype() {
                if is_std_type(ty, ctx) {
                    return true;
                }
                symbol_is_default(ty, overlay, ctx)
            } else {
                true
            }
        },
        symbol_overlay,
        ctx,
    )
}
