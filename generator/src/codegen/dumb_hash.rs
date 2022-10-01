use std::{
    fmt::{Display, Write},
    path::Path,
};

use crate::{
    cat,
    codegen_support::{
        format_utils::{Cond, ExtendedFormat, Fun, Separated},
        type_analysis::is_void_pointer,
        type_query::DeriveData,
    },
    import, symbol_or_value,
};
use crate::{codegen_support::format_utils::SectionWriter, context::Context};
use format_macro::code;
use generator_lib::{
    interner::UniqueStr,
    type_declaration::{TyToken, TypeRef},
    Symbol, SymbolBody,
};

use super::deep_copy::len_paths_deepcopy;

pub fn write_dumb_hash(derives: &mut DeriveData, out: &Path, ctx: &Context) {
    let mut w = SectionWriter::new(
        ctx.create_section("dumb_hash"),
        out.join("src/dumb_hash.rs"),
        true,
        &ctx,
    );

    let mut passthrough = Vec::new();
    for &(symbol_index, _) in &ctx.symbols {
        let &Symbol(name, ref body) = &ctx.reg.symbols[symbol_index];

        match body {
            SymbolBody::Enum { .. } | SymbolBody::Handle { .. } => {
                passthrough.push(name);
                continue;
            }
            SymbolBody::Alias(_)
            | SymbolBody::Redeclaration(_)
            | SymbolBody::Included { .. }
            | SymbolBody::Define { .. }
            | SymbolBody::Basetype { .. }
            | SymbolBody::Constant { .. }
            | SymbolBody::Command { .. }
            | SymbolBody::Funcpointer { .. } => continue,
            _ => {}
        }

        // if derives.is_no_void(name, ctx) == false {
        //     continue;
        // } else
        if derives.is_eq(name, ctx) {
            passthrough.push(name);
            continue;
        }

        match body {
            &SymbolBody::Struct { union, ref members } => {
                let for_union = Cond(
                    union,
                    Fun(|w: &mut SectionWriter| {
                        code!(
                            w,
                            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of::<Self>()).hash(state);
                        );
                        Ok(())
                    }),
                );

                let for_struct = Cond(
                    !union,
                    Fun(|w: &mut SectionWriter| {
                        for d in members {
                            let ptr = cat!("self.", d.name);
                            if d.name == ctx.strings.pNext
                                && (d.ty == ctx.types.void_const_ptr || d.ty == ctx.types.void_ptr)
                            {
                                code!(
                                    w,
                                    hash_pnext(#ptr, state);
                                );
                            } else {
                                fmt_dumb_hash(w, &ptr, &d.ty, 0, &d.metadata.length, ctx);
                            }
                        }
                        Ok(())
                    }),
                );

                code!(
                    w,
                    impl DumbHash for #import!(name) {
                        fn hash<H: Hasher>(&self, state: &mut H) {
                            unsafe {
                                #for_union
                                #for_struct
                            }
                        }
                    }
                );
            }
            _ => unreachable!("{:?}", body),
        }
    }
    let passthrough = Separated::args(passthrough, |w, n| {
        code!(w, #import!(n));
        Ok(())
    });
    code!(
        w,
        dumb_hash_passthrough_impl! {
            u8, i8, u16, i16, u32, i32, u64, i64, u128, i128,
            usize, isize, std::ffi::CStr,
            #passthrough
        }
    )
}

fn fmt_dumb_hash(
    w: &mut SectionWriter,
    name: &dyn Display,
    ty: &TypeRef,
    i: usize,
    len: &[UniqueStr],
    ctx: &Context,
) {
    match ty.as_slice() {
        &[TyToken::BaseType(basetype)] | &[TyToken::Array(_), TyToken::BaseType(basetype)] => {
            if let Some(SymbolBody::Funcpointer { .. }) = ctx.get_symbol(basetype) {
                code!(
                    w,
                    std::ptr::hash(#name as *const (), state);
                )
            } else {
                code!(
                    w,
                    #name.hash(state);
                )
            }
        }
        &[TyToken::Ptr, TyToken::BaseType(basetype)]
        | &[TyToken::Ptr, TyToken::Const, TyToken::BaseType(basetype)] => {
            if let Some(&len) = len.get(0) {
                if len == ctx.strings.null_terminated {
                    code!(
                        w,
                        hash_cstr(#name, state);
                    )
                } else if basetype == ctx.strings.void {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        hash_raw_arr(#name.cast::<u8>(), (#path) as usize, state);
                    )
                } else {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        hash_raw_arr(#name, (#path) as usize, state);
                    )
                }
            } else {
                if is_void_pointer(ty, ctx) {
                    // we have no information, the best we can do is hash the adress
                    code!(
                        w,
                        #name.hash(state);
                    )
                } else {
                    code!(
                        w,
                        hash_ptr(#name, state);
                    )
                }
            }
        }
        &[TyToken::Ptr, ..] => {
            let recurse = Fun(|w| {
                fmt_dumb_hash(
                    w,
                    &"ptr",
                    ty.descend(),
                    i + 1,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    ctx,
                );
                Ok(())
            });
            if let Some(&len) = len.get(0) {
                let len = len_paths_deepcopy(&len, ctx);
                code!(w,
                    let len = (#len) as usize;
                    len.hash(state);
                    for i in 0..len {
                        let ptr = *#name.add(i);
                        #recurse
                    }
                );
            } else {
                code!(
                    w,
                    let ptr = *(#name);
                    #i.hash(state);
                    #recurse
                )
            }
        }
        &[TyToken::Array(Some(const_len)), ..] => {
            let ty = ty.descend();
            let recurse = Fun(|w| {
                fmt_dumb_hash(
                    w,
                    &"ptr",
                    ty.descend(),
                    i + 1,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    ctx,
                );
                Ok(())
            });
            code!(
                w,
                for i in 0..#symbol_or_value!(const_len) as usize {
                    let ptr = &#name[i];
                    #recurse
                }
            )
        }
        _ => unreachable!("{name}: {ty}"),
    }
}