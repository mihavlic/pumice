use std::ops::Deref;

use generator_lib::{
    interner::UniqueStr,
    type_declaration::{BasetypeOrRef, TypeRef},
    ConstantValue, Registry, SymbolBody,
};

use crate::{context::Context, switch};

/// The difference between this and `resolve_alias()` is that this also jumps through "transparent" symbols, such as handles or constants.
pub fn get_underlying_type(name: UniqueStr, ctx: &Context) -> BasetypeOrRef {
    let mut symbol = name;
    loop {
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
                            return BasetypeOrRef::basetype(ctx.strings.uint32_t)
                        }
                        ConstantValue::Symbol(s) => symbol = *s,
                        ConstantValue::String(_) => {
                            return BasetypeOrRef::basetype(ctx.strings.__cstring_constant_type)
                        }
                    }
                }
            }
            // really the only macros that are left are version constants so this is good enough for now
            SymbolBody::Define { .. } => return BasetypeOrRef::basetype(ctx.strings.uint32_t),
            SymbolBody::Redeclaration(_)
            | SymbolBody::Basetype { .. }
            | SymbolBody::Included { .. }
            | SymbolBody::Struct { .. }
            | SymbolBody::Funcpointer { .. } => return BasetypeOrRef::basetype(symbol),
            SymbolBody::Command { .. } => unreachable!(),
        }
    }
}

/// Jumps through as many aliases (Symbol::Alias) as needed and returns the resulting non-alias symbol,
/// in cases where the provided identifier is not an alias it is immediatelly returned back
pub fn get_underlying_symbol<'a>(
    mut name: UniqueStr,
    reg: &'a Registry,
) -> (UniqueStr, &'a SymbolBody) {
    loop {
        let symbol = reg.get_symbol(name).unwrap();
        match symbol {
            &SymbolBody::Alias(of) => {
                name = of;
            }
            _ => return (name, symbol),
        }
    }
}

pub fn is_function_pointer(basetype: UniqueStr, reg: &Registry) -> bool {
    matches!(
        get_underlying_symbol(basetype, reg).1,
        SymbolBody::Funcpointer { .. }
    )
}

pub fn is_std_type(ty: UniqueStr, ctx: &Context) -> bool {
    let s = &ctx.strings;
    switch!(ty;
        s.void, s.int, s.char, s.float, s.double, s.bool,
        s.uint8_t, s.uint16_t, s.uint32_t, s.uint64_t, s.int8_t, s.int16_t, s.int32_t, s.int64_t, s.size_t => true;
        @ false
    )
}

pub trait TypeAnalysis {
    fn is_void_pointer(&self, ctx: &Context) -> bool;
    fn is_function_pointer(&self, ctx: &Context) -> bool;
}

impl TypeAnalysis for TypeRef {
    fn is_void_pointer(&self, ctx: &Context) -> bool {
        self.resolve_alias(ctx)
            .try_ptr_target()
            .map(|ty| ty.resolve_alias(ctx).deref() == ctx.types.void.deref())
            .unwrap_or(false)
    }
    fn is_function_pointer(&self, ctx: &Context) -> bool {
        self.resolve_alias(ctx)
            .try_only_basetype()
            .map(|name| is_function_pointer(name, ctx))
            .unwrap_or(false)
    }
}
