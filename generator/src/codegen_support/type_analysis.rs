use std::fmt::Write;

use generator_lib::{
    interner::UniqueStr,
    type_declaration::{TyToken, TypeRef},
    ConstantValue, RedeclarationMethod, Registry, SymbolBody,
};

use crate::{context::Context, switch};

use super::format_utils::{ExtendedFormat, SectionWriter};

#[derive(Clone, Copy, Debug)]
pub enum UnderlyingType {
    Basetype(UniqueStr),
    CString,
}

impl UnderlyingType {
    pub fn try_basetype(&self) -> Option<UniqueStr> {
        match self {
            UnderlyingType::Basetype(b) => Some(*b),
            UnderlyingType::CString => None,
        }
    }
}

impl ExtendedFormat for UnderlyingType {
    fn efmt(self, writer: &mut SectionWriter) -> std::fmt::Result {
        match self {
            UnderlyingType::Basetype(b) => crate::import!(b).efmt(writer),
            UnderlyingType::CString => write!(writer, "&std::ffi::CStr"),
        }
    }
}

// Get the underlying type of a symbol.
// The difference between this and `resolve_alias()` is that this also jumps through "transparent" symbols, such as handles or constants.
pub fn get_underlying_type(name: UniqueStr, ctx: &Context) -> UnderlyingType {
    let mut symbol = name;
    loop {
        if is_std_type(symbol, &ctx) {
            return UnderlyingType::Basetype(symbol);
        }

        let top = ctx.get_symbol(symbol).unwrap();
        match top {
            SymbolBody::Alias(of) => symbol = *of,
            SymbolBody::Enum { ty, .. } => symbol = *ty,
            SymbolBody::Handle { .. } => symbol = ctx.strings.uint64_t, // the underlying type of all handles is this
            SymbolBody::Constant { ty, val } => {
                if let Some(ty) = ty {
                    symbol = *ty;
                } else {
                    match val {
                        ConstantValue::Bitpos(_) => unreachable!(),
                        ConstantValue::Literal(_) | ConstantValue::Expression(_) => {
                            return UnderlyingType::Basetype(ctx.strings.uint32_t)
                        }
                        ConstantValue::Symbol(s) => symbol = *s,
                        ConstantValue::String(_) => return UnderlyingType::CString,
                    }
                }
            }
            // really the only macros that are left are version constants so this is good enough for now
            SymbolBody::Define { .. } => return UnderlyingType::Basetype(ctx.strings.uint32_t),
            SymbolBody::Redeclaration(_)
            | SymbolBody::Basetype { .. }
            | SymbolBody::Included { .. }
            | SymbolBody::Struct { .. }
            | SymbolBody::Funcpointer { .. } => return UnderlyingType::Basetype(symbol),
            SymbolBody::Command { .. } => unreachable!(),
        }
    }
}

pub fn try_ptr_target<'a>(ty: &'a TypeRef, ctx: &'a Context) -> Option<&'a TypeRef> {
    match ty.as_slice() {
        &[TyToken::BaseType(mut basetype)] => loop {
            match ctx.get_symbol(basetype) {
                Some(SymbolBody::Alias(of)) => basetype = *of,
                Some(SymbolBody::Redeclaration(RedeclarationMethod::Type(ty))) => {
                    return try_ptr_target(ty, ctx);
                }
                _ => return None,
            }
        },
        tokens @ &[TyToken::Ptr, ..] => {
            return Some(TypeRef::from_slice(&tokens[1..]));
        }
        _ => return None,
    }
}

// jumps through as many aliases (Symbol::Alias) as needed and returns the resulting non-alias symbol,
// in cases where the provided identifier is not an alias it is immediatelly returned back
pub fn resolve_alias<'a>(alias: UniqueStr, reg: &'a Registry) -> (UniqueStr, &'a SymbolBody) {
    let mut next = alias;
    loop {
        let symbol = reg
            .get_symbol(next)
            .unwrap_or_else(|| panic!("'{}' Not in registry", { next }));
        match symbol {
            &SymbolBody::Alias(of) => {
                next = of;
            }
            _ => return (next, symbol),
        }
    }
}

// whether the type is provided from the rust standard library and as such has no entry in the Registry
pub fn is_std_type(ty: UniqueStr, ctx: &Context) -> bool {
    let s = &ctx.strings;
    switch!(ty;
        s.void, s.int, s.char, s.float, s.double, s.bool,
        s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => true;
        @ false
    )
}

pub fn is_void_pointer(ty: &TypeRef, ctx: &Context) -> bool {
    try_ptr_target(ty, ctx)
        .map(|ty| match ty.as_slice() {
            &[TyToken::BaseType(mut basetype)]
            | &[TyToken::Const, TyToken::BaseType(mut basetype)] => loop {
                if basetype == ctx.strings.void {
                    break true;
                }
                if let Some(SymbolBody::Alias(of)) = ctx.get_symbol(basetype) {
                    basetype = *of;
                } else {
                    break false;
                }
            },
            _ => false,
        })
        .unwrap_or(false)
}
