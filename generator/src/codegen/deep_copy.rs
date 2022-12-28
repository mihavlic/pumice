use std::{fmt::Display, path::Path, rc::Rc};

use crate::{
    cat,
    codegen_support::{
        format_utils::SymbolOrValue, type_analysis::TypeAnalysis, type_query::DeriveData,
    },
    import,
};
use crate::{codegen_support::format_utils::SectionWriter, context::Context};
use codewrite::{CFmt, Cond, Fun, Iter, Separated};
use codewrite_macro::code;
use generator_lib::{
    interner::UniqueStr,
    smallvec::SmallVec,
    type_declaration::{TyToken, TypeRef},
    Symbol, SymbolBody,
};

use super::wrappers::{len_paths, VarSource};

pub fn write_deep_copy(derives: &mut DeriveData, out: &Path, ctx: &Rc<Context>) {
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
        // }

        match body {
            SymbolBody::Enum { .. }
            | SymbolBody::Handle { .. }
            | SymbolBody::Struct { union: true, .. } => value_types.push(name),
            SymbolBody::Struct {
                union: false,
                members,
            } => {
                if derives.is_value_type(name, ctx) {
                    value_types.push(name);
                } else {
                    let non_value = members
                        .iter()
                        .filter(|m| !derives.type_is_value(&m.ty, ctx))
                        .collect::<SmallVec<[_; 8]>>();

                    let measure = Iter::new(&non_value, |w: &mut SectionWriter, &d| {
                        let ptr = cat!("self.", d.name);
                        if d.name == ctx.strings.pNext
                            && (d.ty == ctx.types.void_const_ptr || d.ty == ctx.types.void_ptr)
                        {
                            code!(
                                w,
                                measure.measure_pnext($ptr);
                            );
                        } else {
                            fmt_deepcopy_measure(w, &ptr, &d.ty, &d.metadata.length, 0, ctx);
                        }
                    });
                    let copy = Iter::new(&non_value, |w: &mut SectionWriter, &d| {
                        let ptr = cat!("self.", d.name);
                        let copy = cat!("(*copy).", d.name);
                        if d.name == ctx.strings.pNext
                            && (d.ty == ctx.types.void_const_ptr || d.ty == ctx.types.void_ptr)
                        {
                            let cast =
                                Cond::new(d.ty.as_slice()[1] == TyToken::Const, "as *const _");
                            code!(
                                w,
                                $copy = writer.write_pnext($ptr) $cast;
                            );
                        } else {
                            fmt_deepcopy_copy(w, &ptr, &copy, &d.ty, &d.metadata.length, 0, ctx);
                        }
                    });
                    code!(
                        w,
                        unsafe impl DeepCopy for $import!(name) {
                            fn measure(&self, measure: &mut CopyMeasure) {
                                unsafe {
                                    $measure
                                }
                            }
                            unsafe fn copy(&self, copy: *mut Self, writer: &mut CopyWriter) {
                                $copy
                            }
                        }
                    );
                }
            }
            _ => unreachable!(),
        }
    }
    let names = Separated::args(value_types, |w: &mut SectionWriter, n| {
        code!(w, $import!(n));
    });
    code!(
        w,
        value_type_copy_impl! {
            u8, i8, u16, i16, u32, i32, u64, i64, u128, i128,
            f32, f64,
            usize, isize,
            $names
        }
    )
}

fn fmt_deepcopy_measure(
    w: &mut SectionWriter,
    name: &dyn Display,
    ty: &TypeRef,
    len: &[UniqueStr],
    depth: usize,
    ctx: &Context,
) {
    match ty.resolve_alias(ctx).as_slice() {
        &[TyToken::BaseType(basetype)] => {
            if ty.is_void_pointer(ctx) {
                // do nothing, we have no information
            } else {
                if let Some(SymbolBody::Funcpointer { .. }) = ctx.get_symbol(basetype) {
                    // was already copied with the containing struct
                } else {
                    code!(
                        w,
                        measure.measure_ptr(&$name);
                    )
                }
            }
        }
        &[TyToken::Ptr, TyToken::BaseType(basetype)]
        | &[TyToken::Ptr, TyToken::Const, TyToken::BaseType(basetype)] => {
            if let Some(&len) = len.get(0) {
                if len == ctx.strings.null_terminated {
                    code!(
                        w,
                        measure.measure_cstr($name);
                    )
                } else if basetype == ctx.strings.void {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        measure.measure_arr_ptr($name.cast::<u8>(), ($path) as usize);
                    )
                } else {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        measure.measure_arr_ptr($name, ($path) as usize);
                    )
                }
            } else {
                if ty.is_void_pointer(ctx) {
                    // do nothing, we have no information
                } else {
                    code!(
                        w,
                        measure.measure_ptr($name);
                    )
                }
            }
        }
        &[TyToken::Ptr, ..] => {
            let ty = ty.descend();
            let measure = Fun::new(|w: &mut SectionWriter| {
                fmt_deepcopy_measure(
                    w,
                    &"ptr",
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    depth + 1,
                    ctx,
                );
            });
            if let Some(&len) = len.get(0) {
                let len = len_paths_deepcopy(&len, ctx);
                code!(w,
                    let len = ($len) as usize;
                    measure.alloc_arr::<$import!(ty)>(len);
                    for i in 0..len {
                        let ptr = *$name.add(i);
                        $measure
                    }
                );
            } else {
                code!(
                    w,
                    let ptr = *($name);
                    measure.alloc::<$import!(ty)>();
                    $measure
                )
            }
        }
        &[TyToken::Array(Some(const_len)), ..] => {
            let letter = ['i', 'j', 'k', 'l', 'm'][depth];
            let ty = ty.descend();
            let measure = Fun::new(|w: &mut SectionWriter| {
                fmt_deepcopy_measure(
                    w,
                    &cat!(name, '[', letter, ']'),
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    depth + 1,
                    ctx,
                );
            });
            code!(
                w,
                for $letter in 0..$(SymbolOrValue(const_len)) as usize {
                    $measure
                }
            )
        }
        _ => unimplemented!("{name}: {:?}", ty.as_slice()),
    }
}

fn fmt_deepcopy_copy(
    w: &mut SectionWriter,
    name: &dyn Display,
    copy: &dyn Display,
    ty: &TypeRef,
    len: &[UniqueStr],
    depth: usize,
    ctx: &Context,
) {
    match ty.resolve_alias(ctx).as_slice() {
        &[TyToken::BaseType(basetype)] => {
            if ty.is_void_pointer(ctx) {
                // do nothing, we have no information
            } else {
                if let Some(SymbolBody::Funcpointer { .. }) = ctx.get_symbol(basetype) {
                    // was already copied with the containing struct
                } else {
                    code!(
                        w,
                        $name.copy(&mut $copy, writer);
                    )
                }
            }
        }
        &[TyToken::Ptr, TyToken::BaseType(basetype)]
        | &[TyToken::Ptr, TyToken::Const, TyToken::BaseType(basetype)] => {
            if let Some(&len) = len.get(0) {
                if len == ctx.strings.null_terminated {
                    code!(
                        w,
                        $copy = writer.write_cstr($name);
                    )
                } else if basetype == ctx.strings.void {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        $copy = writer.write_arr_ptr($name.cast::<u8>(), ($path) as usize).cast::<c_void>();
                    )
                } else {
                    let path = len_paths_deepcopy(&len, ctx);
                    code!(
                        w,
                        $copy = writer.write_arr_ptr($name, ($path) as usize);
                    )
                }
            } else {
                if ty.is_void_pointer(ctx) {
                    // do nothing, we have no information
                } else {
                    code!(
                        w,
                        $copy = writer.write_ptr($name);
                    )
                }
            }
        }
        slice @ &[TyToken::Ptr, ..] => {
            let cast = Cond::new(slice[1] == TyToken::Const, ".cast_mut()");
            let ty = ty.descend();
            let recurse = Fun::new(|w: &mut SectionWriter| {
                fmt_deepcopy_copy(
                    w,
                    &"ptr",
                    &"*copy",
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    depth + 1,
                    ctx,
                );
            });
            if let Some(&len) = len.get(0) {
                let len = len_paths_deepcopy(&len, ctx);
                code!(w,
                    let len = ($len) as usize;
                    $copy = writer.alloc_arr::<$import!(ty)>(len);
                    for i in 0..len {
                        let ptr = *$name.add(i);
                        let copy = $copy.add(i) $cast;
                        $recurse
                    }
                );
            } else {
                code!(
                    w,
                    let ptr = *($name);
                    $copy = writer.alloc::<$import!(ty)>();
                    $recurse
                )
            }
        }
        &[TyToken::Array(Some(const_len)), ..] => {
            let ty = ty.descend();
            let recurse = Fun::new(|w: &mut SectionWriter| {
                fmt_deepcopy_copy(
                    w,
                    &"*ptr",
                    &"*copy",
                    ty,
                    len.split_first().map(|(_, e)| e).unwrap_or(&[]),
                    depth + 1,
                    ctx,
                );
            });
            code!(
                w,
                for i in 0..$(SymbolOrValue(const_len)) as usize {
                    let ptr = &$name[i];
                    let copy = &mut $copy[i];
                    $recurse
                }
            )
        }
        _ => unreachable!("{name}: {ty}"),
    }
}

pub fn len_paths_deepcopy<'a>(
    len: &'a UniqueStr,
    ctx: &'a Context,
) -> impl CFmt<SectionWriter> + 'a {
    len_paths(
        len.resolve(),
        |s| {
            if ctx.get_symbol(s).is_some() {
                VarSource::Import
            } else {
                VarSource::InSelf
            }
        },
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
