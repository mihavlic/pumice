use std::{
    fmt::{Display, Write},
    path::Path,
};

use crate::{
    cat,
    codegen_support::{
        derives::DeriveData,
        format_utils::{Cond, ExtendedFormat, Fun, Separated},
    },
    import,
};
use crate::{
    codegen_support::format_utils::{Iter, SectionWriter},
    context::Context,
};
use format_macro::code;
use generator_lib::{
    interner::UniqueStr,
    smallvec::SmallVec,
    type_declaration::{TyToken, TypeRef},
    Symbol, SymbolBody,
};

use super::wrappers::{len_paths, VarSource};

pub fn write_deep_copy(derives: &mut DeriveData, out: &Path, ctx: &Context) {
    let mut w = SectionWriter::new(
        ctx.create_section("deep_copy"),
        out.join("src/deep_copy.rs"),
        true,
        &ctx,
    );

    let mut value_types = Vec::new();
    for &(symbol_index, _) in &ctx.symbols {
        let &Symbol(name, ref body) = &ctx.reg.symbols[symbol_index];

        match body {
            SymbolBody::Alias(_) |
            SymbolBody::Redeclaration(_) |
            SymbolBody::Included { .. } |
            SymbolBody::Define { .. } |
            SymbolBody::Basetype { .. } |
            SymbolBody::Constant { .. } |
            SymbolBody::Command { .. } |
            // function pointers are just aliases so we can't implement anything on them
            SymbolBody::Funcpointer { .. } => continue,
            _ => {
                if derives.is_no_void(name, ctx) == false {
                    continue;
                }
            }
        }

        match body {
            SymbolBody::Enum { .. } | SymbolBody::Handle { .. } => value_types.push(name),
            SymbolBody::Struct { union, members } => {
                if derives.is_value_type(name, ctx) {
                    value_types.push(name);
                } else {
                    assert!(*union == false);

                    let non_value = members
                        .iter()
                        .filter(|m| !derives.type_is_value(&m.ty, ctx))
                        .collect::<SmallVec<[_; 8]>>();

                    let measure = Iter(&non_value, |w, d| {
                        let ptr = cat!("self.", d.name);
                        if d.name == ctx.strings.pNext
                            && (d.ty == ctx.types.void_const_ptr || d.ty == ctx.types.void_ptr)
                        {
                            code!(
                                w,
                                measure.measure_pnext(#ptr);
                            );
                        } else {
                            fmt_deepcopy_measure(w, &ptr, &d.ty, &d.metadata.length, ctx);
                        }
                    });
                    let copy = Iter(&non_value, |w, d| {
                        let ptr = cat!("self.", d.name);
                        let copy = cat!("(*copy).", d.name);
                        if d.name == ctx.strings.pNext
                            && (d.ty == ctx.types.void_const_ptr || d.ty == ctx.types.void_ptr)
                        {
                            let cast = Cond(d.ty.as_slice()[1] == TyToken::Const, "as *const _");
                            code!(
                                w,
                                #copy = writer.write_pnext(#ptr) #cast;
                            );
                        } else {
                            fmt_deepcopy_copy(w, &ptr, &copy, &d.ty, &d.metadata.length, ctx);
                        }
                    });
                    code!(
                        w,
                        unsafe impl DeepCopy for #import!(name) {
                            fn measure(&self, measure: &mut CopyMeasure) {
                                unsafe {
                                    #measure
                                }
                            }
                            unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
                                #copy
                            }
                        }
                    );
                }
            }
            _ => unreachable!(),
        }
    }
    let names = Separated::args(value_types, |w, n| {
        code!(w, #import!(n));
        Ok(())
    });
    code!(
        w,
        value_type_copy_impl! {
            u8, i8, u16, i16, u32, i32, u64, i64, u128, i128,
            f32, f64,
            usize, isize,
            #names
        }
    )
}

fn fmt_deepcopy_measure(
    w: &mut SectionWriter,
    name: &dyn Display,
    ty: &TypeRef,
    len: &[UniqueStr],
    ctx: &Context,
) {
    match ty.as_slice() {
        &[TyToken::BaseType(_)] => {
            code!(
                w,
                measure.measure_ptr(&#name);
            )
        }
        &[TyToken::Ptr, TyToken::BaseType(basetype)]
        | &[TyToken::Ptr, TyToken::Const, TyToken::BaseType(basetype)] => {
            if let Some(&len) = len.get(0) {
                if len == ctx.strings.null_terminated {
                    code!(
                        w,
                        measure.measure_cstr(#name);
                    )
                } else if basetype == ctx.strings.void {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        measure.measure_raw_arr(#name.cast::<u8>(), (#path) as usize);
                    )
                } else {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        measure.measure_raw_arr(#name, (#path) as usize);
                    )
                }
            } else {
                code!(
                    w,
                    measure.measure_ptr(#name);
                )
            }
        }
        &[TyToken::Ptr, ..] => {
            let ty = ty.descend();
            let measure = Fun(|w| {
                fmt_deepcopy_measure(
                    w,
                    &"ptr",
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    ctx,
                );
                Ok(())
            });
            if let Some(&len) = len.get(0) {
                let len = len_paths_deepcopy(&len, ctx);
                code!(w,
                    let len = (#len) as usize;
                    measure.alloc_raw_arr::<#import!(ty)>(len);
                    for i in 0..len {
                        let ptr = *#name.add(i);
                        #measure
                    }
                );
            } else {
                code!(
                    w,
                    let ptr = *(#name);
                    measure.alloc_raw::<#import!(ty)>();
                    #measure
                )
            }
        }
        _ => unreachable!("{name}: {ty}"),
    }
}

fn fmt_deepcopy_copy(
    w: &mut SectionWriter,
    name: &dyn Display,
    copy: &dyn Display,
    ty: &TypeRef,
    len: &[UniqueStr],
    ctx: &Context,
) {
    match ty.as_slice() {
        &[TyToken::BaseType(_)] => {
            code!(
                w,
                #name.copy(&mut #copy, writer);
            )
        }
        &[TyToken::Ptr, TyToken::BaseType(basetype)]
        | &[TyToken::Ptr, TyToken::Const, TyToken::BaseType(basetype)] => {
            if let Some(&len) = len.get(0) {
                if len == ctx.strings.null_terminated {
                    code!(
                        w,
                        #copy = writer.write_cstr(#name);
                    )
                } else if basetype == ctx.strings.void {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        #copy = writer.write_raw_arr(#name.cast::<u8>(), (#path) as usize).cast::<c_void>();
                    )
                } else {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        #copy = writer.write_raw_arr(#name, (#path) as usize);
                    )
                }
            } else {
                code!(
                    w,
                    #copy = writer.write_ptr(#name);
                )
            }
        }
        &[TyToken::Ptr, ..] => {
            let ty = ty.descend();
            let recurse = Fun(|w| {
                fmt_deepcopy_copy(
                    w,
                    &"ptr",
                    &"*copy",
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    ctx,
                );
                Ok(())
            });
            if let Some(&len) = len.get(0) {
                let len = len_paths_deepcopy(&len, ctx);
                code!(w,
                    let len = (#len) as usize;
                    *addr_of_mut!(#copy) = writer.alloc_raw_arr::<#import!(ty)>(len);
                    for i in 0..len {
                        let ptr = *#name.add(i);
                        let copy = #copy.add(i) as *mut _;
                        #recurse
                    }
                );
            } else {
                code!(
                    w,
                    let ptr = *(#name);
                    *addr_of_mut!(#copy) = writer.alloc_raw::<#import!(ty)>();
                    let copy = #copy;
                    #recurse
                )
            }
        }
        _ => unreachable!("{name}: {ty}"),
    }
}

fn len_paths_deepcopy<'a>(len: &'a UniqueStr, ctx: &'a Context) -> impl ExtendedFormat + 'a {
    len_paths(
        len.resolve(),
        |_| VarSource::InSelf,
        |s| {
            // QUIRK rasterization_samples is not an integer but an enum whose variants are made to interpret correctly as integers
            if s == "rasterization_samples" {
                "rasterization_samples.0"
            } else {
                s
            }
        },
        ".deref().",
        ctx,
    )
}
