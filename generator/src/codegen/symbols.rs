use std::{cell::Cell, collections::HashMap};

use crate::{
    cat,
    codegen::wrappers::fmt_command_wrapper,
    codegen_support::{
        format_utils::SectionWriter,
        get_command_kind,
        type_analysis::{
            get_underlying_symbol, get_underlying_type, is_function_pointer, TypeAnalysis,
        },
        type_query::DeriveData,
        AddedVariants, CommandKind,
    },
    cond,
    context::Context,
    cstring, doc_boilerplate, fun, fun_once, import, import_str, string,
};
use codewrite::{Cond, Iter, Separated};
use codewrite_macro::code;
use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::{TyToken, TypeRef},
    ConstantValue, Declaration, RedeclarationMethod, SymbolBody,
};
use std::fmt::Write;

pub fn write_symbol(
    w: &mut SectionWriter,
    name: UniqueStr,
    body: &SymbolBody,
    derives: &mut DeriveData,
    added_variants: &HashMap<UniqueStr, Vec<AddedVariants>>,
    ctx: &Context,
) {
    macro_rules! select {
        ($cond:expr, $true:expr, $false:expr) => {
            if $cond {
                $true
            } else {
                $false
            }
        };
    }

    let bitflags_impl = import_str!("bitflags_impl", ctx);
    let enum_impl = import_str!("enum_impl", ctx);
    let non_dispatchable_handle = import_str!("non_dispatchable_handle", ctx);
    let dispatchable_handle = import_str!("dispatchable_handle", ctx);

    match body {
        &SymbolBody::Alias(of) => {
            let target = get_underlying_symbol(of, &ctx);
            match target.1 {
                SymbolBody::Alias { .. } | SymbolBody::Define { .. } => {
                    unreachable!();
                }
                SymbolBody::Constant { .. } => {
                    let ty = get_underlying_type(target.0, &ctx);
                    code!(
                        w,
                        pub const $name: $import!(ty) = $import!(target.0);
                    );
                    return;
                }
                body @ SymbolBody::Command {
                    success_codes: _,
                    error_codes: _,
                    return_type,
                    params,
                } => {
                    fmt_global_command(w, name, target.0, params, return_type, ctx);
                    fmt_global_command_wrapper(w, name, body, params, ctx);
                    return;
                }
                _ => {}
            };

            code!(
                w, (),
                $doc_boilerplate!(name)
                pub type $name = $import!(of);
            );
        }
        SymbolBody::Redeclaration(method) => match method {
            RedeclarationMethod::Type(ty) => {
                code!(
                    w,
                    pub type $name = $import!(ty);
                );
            }
            RedeclarationMethod::Custom(fun) => {
                fun(w).unwrap();
            }
        },
        // there is nothing to do with defines in rust, just skip them
        SymbolBody::Define { body } => {
            // FIXME burn this code down

            let mut code = String::new();
            let mut chars = body.chars().peekable();

            // remove comments
            'outer: while let Some(next) = chars.next() {
                match next {
                    '/' => {
                        if let Some('/') = chars.peek() {
                            while let Some(next) = chars.next() {
                                if next == '\n' {
                                    break;
                                }
                            }
                            continue 'outer;
                        }
                    }
                    _ => {}
                }
                code.push(next);
            }

            // String::remove_matches is unstable so this is what you get
            code = code.replace("#define", "");
            code = code.replace(name.resolve_original(), "");
            code = code.replace(
                "VK_MAKE_VIDEO_STD_VERSION",
                "crate::extensions::video_common::make_video_std_version",
            );
            code = code.replace("VK_MAKE_API_VERSION", "crate::vk10::make_api_version");
            code = code.replace("VK_API_VERSION_VARIANT", "crate::vk10::api_version_variant");
            code = code.replace("VK_API_VERSION_MAJOR", "crate::vk10::api_version_major");
            code = code.replace("VK_API_VERSION_MINOR", "crate::vk10::api_version_minor");
            code = code.replace("VK_API_VERSION_PATCH", "crate::vk10::api_version_patch");
            code = code.replace(
                "VK_HEADER_VERSION",
                // the constant gets renamed and we need to know its new name
                "VK_HEADER_VERSION".intern(ctx).resolve(),
            );

            code!(
                w,
                pub const $name: u32 = $code;
            );
        }
        SymbolBody::Included { .. } => {
            unreachable!("[{}]", name);
        }
        SymbolBody::Basetype { .. } => {
            // at this point the meaning of a basetype is changed to be a primitive type, as such no code generation is required
        }
        &SymbolBody::Handle {
            object_type,
            dispatchable,
        } => {
            let raw = name.resolve_original();
            let handle = select!(dispatchable, non_dispatchable_handle, dispatchable_handle);

            code!(
                w,
                $(handle)! (
                    $name, $object_type, $string!(raw),
                    $cat!("\"[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/", raw, ".html)\"")
                );
            );
        }
        SymbolBody::Funcpointer { ret, args } => {
            let preamble = fun!(|w| {
                fmt_command_preamble(
                    w,
                    "",
                    args.iter().map(|p| p.0),
                    args.iter().map(|p| &*p.1),
                    true,
                    "",
                    ret,
                    ctx,
                )
            });
            code!(
                w,
                $doc_boilerplate!(name)
                pub type $name = unsafe extern "system" $preamble;
            );
        }
        &SymbolBody::Struct { union, ref members } => {
            let keyword = match union {
                true => "union",
                false => "struct",
            };
            let fields = Separated::args(members.iter(), |w: &mut SectionWriter, decl| {
                if decl.ty.is_function_pointer(ctx) {
                    code!(w, pub $(decl.name): Option<$import!(&decl.ty)>);
                } else {
                    code!(w, pub $(decl.name): $import!(&decl.ty));
                }
            });

            let default_members = Separated::args(members.iter(), |w: &mut SectionWriter, decl| {
                let default = fun!(|w| {
                    fmt_default_value(w, &decl, ctx);
                });
                code!(w, $(decl.name): $default);
            });
            let struct_default = fun!(|w| {
                if !union {
                    code!(
                        w,
                        Self {
                            $default_members
                        }
                    );
                }
            });

            let union_default = fun!(|w| {
                if union {
                    code!(w, unsafe { std::mem::zeroed() });
                }
            });

            let copy = Cond::new(derives.is_copy(name, ctx), ", Copy");
            let eq = Cond::new(derives.is_eq(name, ctx), ", PartialEq, Eq, Hash");
            let zeroable = derives.is_defaultable(name, ctx);

            code!(
                w,
                #[derive(Clone $copy $eq)]
                #[repr(C)]
                $doc_boilerplate!(name)
                pub $keyword $name {
                    $fields
                }
                $cond!(zeroable => |w| code!(w,
                    impl Default for $name {
                        fn default() -> Self {
                            $struct_default
                            $union_default
                        }
                    }
                ))
            );
        }
        SymbolBody::Constant { val, .. } => {
            let ty = get_underlying_type(name, &ctx);

            let val = fun!(|w| {
                use ConstantValue::*;
                match val {
                    Bitpos(p) => code!(w, 1 << $p),
                    Literal(i) => code!(w, $i),
                    Expression(e) => code!(w, $e),
                    Symbol(s) => code!(w, $import!(*s)),
                    String(s) => code!(w, $cstring!(s)),
                }
            });

            code!(w, pub const $name: $import!(ty) = $val;)
        }
        SymbolBody::Enum {
            ty,
            members,
            bitmask,
        } => {
            let tmp = get_underlying_type(*ty, &ctx);
            let ty = import!(tmp);

            let eq = select!(derives.is_eq(name, ctx), ", Eq, Hash", "");

            let supl = added_variants.get(&name);

            code!(
                w,
                $doc_boilerplate!(name)
                #[derive(Clone, Copy, PartialEq $eq, Default)]
                #[repr(transparent)]
                pub struct $name(pub $ty);
            );

            let member_iter = members.iter().map(|a| (name, a.0, &a.1));
            let supl_raw_iter = supl.into_iter().flatten();
            let supl_iter = supl_raw_iter
                .filter(|a| a.applicable)
                .flat_map(|a| a.variants.iter().map(|b| (a.source_section, b.0, b.1)));

            // Vec<source section, name, value>
            let mut members = member_iter.chain(supl_iter).collect::<Vec<_>>();

            // Vulkan enums allow for some variant to be an alias of another these are mostly used
            // for backwards compatibility when some enum is promoted to core, the _KHR and such
            // variants remain. However if we are only generating the extensions which used to have
            // these variants natively we may happen to not generate the core version that currently
            // has the actual variants of which these are aliases. Thus we only "softly" no-generate
            // these variants and here we look for aliases to softly removed ("not applicable")
            // variants and if this happens we replace the alias to its supposed value
            for (_, _, val) in &mut members {
                'propagate: loop {
                    match val {
                        ConstantValue::Symbol(target) => {
                            for added in supl.into_iter().flatten() {
                                for &(name, other_val) in &added.variants {
                                    if name == *target {
                                        if !added.applicable {
                                            // if the variant has a value that is an alias of another variant,
                                            // we must "inline" the variant, we then loop and match the value again
                                            // because it is possible that it's just another alias of a not applicable variant
                                            *val = other_val;
                                            continue 'propagate;
                                        } else {
                                            break 'propagate;
                                        }
                                    }
                                }
                            }

                            break;
                        }
                        _ => break,
                    }
                }
            }

            // we can't straight out remove() any duplicate variants when we first encounter them because
            // later variants may be aliases of the removed variants and it would make them dangle
            // thus we just replace their source extension with this marker
            let deleted = ctx.strings.__RESERVED_INVALID_PLACEHOLDER;

            for i in 0..members.len() {
                let (_, a_name, mut a_val) = members[i];

                while let ConstantValue::Symbol(to) = a_val {
                    a_val = members
                        .iter()
                        .find(|m| m.1 == *to)
                        .unwrap_or_else(|| {
                            panic!("Variant {} not in {name}!", to.resolve_original())
                        })
                        .2;
                }

                for j in (i + 1)..members.len() {
                    let (b_ext, b_name, mut b_val) = members[j];

                    if b_ext == deleted {
                        continue;
                    }

                    if a_name.eq_resolve(b_name) {
                        while let ConstantValue::Symbol(to) = b_val {
                            b_val = members
                                .iter()
                                .find(|m| m.1 == *to)
                                .unwrap_or_else(|| {
                                    panic!("Variant {} not in {name}!", to.resolve_original())
                                })
                                .2;
                        }

                        if a_val == b_val {
                            members[j].0 = deleted;
                        } else {
                            panic!(
                                "Duplicates '{}' and '{}' should share a value!",
                                a_name.resolve_original(),
                                b_name.resolve_original()
                            )
                        }
                    }
                }
            }

            // only now we can safely delete all the duplicate variants
            members.retain(|m| m.0 != deleted);

            let state = Cell::new(name);
            let variants = Iter::new(&members, |w: &mut SectionWriter, &(ext, name, val)| {
                if state.get() != ext {
                    writeln!(w, "/// {ext}").unwrap();
                    state.set(ext);
                }

                use ConstantValue::*;
                match val {
                    Bitpos(pos) => code!(w, pub const $name: Self = Self(1 << $pos);),
                    Literal(val) => code!(w, pub const $name: Self = Self($val);),
                    Expression(str) => code!(w, pub const $name: Self = Self($str);),
                    Symbol(alias) => code!(w, pub const $name: Self = Self::$alias;),
                    String(_) => unreachable!(),
                }
            });

            if !members.is_empty() {
                code!(
                    w,
                    impl $name {
                        $variants
                    }
                );
            }

            let variants = Separated::display(
                members
                    .iter()
                    .filter(|m| !matches!(m.2, ConstantValue::Symbol(_)))
                    .map(|m| m.1),
                ",",
            );
            if *bitmask {
                let mut all = 0;
                for &(_, _, val) in &members {
                    match val {
                        &ConstantValue::Bitpos(v) => all |= 1 << v,
                        &ConstantValue::Literal(v) => all |= v,
                        ConstantValue::Expression(_) => unreachable!(),
                        ConstantValue::Symbol(_) => {}
                        ConstantValue::String(_) => {}
                    }
                }
                let all = fun!(|w| write!(w, "0x{:x}", all).unwrap(); );

                code!(
                    w,
                    $(bitflags_impl)! {
                        $name: $ty, $all, $variants
                    }
                )
            } else {
                code!(
                    w,
                    $(enum_impl)! {
                        $name: $ty, $variants
                    }
                )
            }
        }
        SymbolBody::Command {
            success_codes: _,
            error_codes: _,
            return_type,
            params,
        } => {
            fmt_global_command(w, name, name, params, return_type, ctx);
            fmt_global_command_wrapper(w, name, body, params, ctx);
        }
    }
}

fn fmt_global_command_wrapper(
    w: &mut SectionWriter,
    name: UniqueStr,
    body: &SymbolBody,
    params: &Vec<Declaration>,
    ctx: &Context,
) {
    let kind = get_command_kind(params, ctx);
    let wrapper_type = match kind {
        CommandKind::Entry => "EntryWrapper",
        CommandKind::Instance => "InstanceWrapper",
        CommandKind::Device => "DeviceWrapper",
    };
    let wrapper_body = fun_once!(move |w| {
        fmt_command_wrapper(w, name, body, kind, ctx);
    });

    code!(
        w,
        #[cfg(feature = "wrappers")]
        impl $import_str!(wrapper_type, ctx) {
            $wrapper_body
        }
    );
}

fn fmt_global_command(
    w: &mut SectionWriter,
    name: UniqueStr,
    fptr_name: UniqueStr,
    params: &Vec<Declaration>,
    return_type: &TypeRef,
    ctx: &Context,
) {
    let preamble = fun!(|w| {
        fmt_command_preamble(
            w,
            name.resolve(),
            params.iter().map(|p| p.name),
            params.iter().map(|p| &*p.ty),
            true,
            "",
            return_type,
            ctx,
        )
    });
    let table = match get_command_kind(&params, ctx) {
        CommandKind::Entry => "GLOBAL_ENTRY_TABLE",
        CommandKind::Instance => "GLOBAL_INSTANCE_TABLE",
        CommandKind::Device => "GLOBAL_DEVICE_TABLE",
    };
    let params = Separated::display(params.iter().map(|p| p.name), ",");

    code!(
        w,
        #[track_caller]
        #[cfg(all(feature = "global", feature = "raw"))]
        $doc_boilerplate!(name)
        pub unsafe $preamble {
            ($import_str!(table, ctx).$fptr_name.unwrap())(
                $params
            )
        }
    );
}

pub fn fmt_command_preamble<'a>(
    w: &mut SectionWriter,
    name: &str,
    param_names: impl Iterator<Item = UniqueStr> + Clone,
    param_types: impl Iterator<Item = &'a TypeRef> + Clone,
    output_names: bool,
    self_str: &str,
    return_type: &TypeRef,
    ctx: &Context,
) {
    let iter = param_names.into_iter().zip(param_types.into_iter());
    let args = Separated::args(iter, |w: &mut SectionWriter, (name, ty)| {
        if output_names {
            code!(w, $name: $import!(ty));
        } else {
            code!(w, $import!(ty));
        }
    });

    code!(w, iter, fn $name($self_str $args));

    if return_type == &*ctx.types.void {
        return;
    }

    code!(w, iter, -> $import!(return_type));
}
pub fn fmt_default_value<'a>(w: &mut SectionWriter, decl: &'a Declaration, ctx: &Context) {
    fmt_default_value_with_overlay(w, decl, &decl.ty, ctx)
}
/// Same functionality as `get_default_value` but uses a different type than the declaration's after not matching sType.
pub fn fmt_default_value_with_overlay(
    w: &mut SectionWriter,
    decl: &Declaration,
    overlay: &TypeRef,
    ctx: &Context,
) {
    if decl.name == ctx.strings.sType
        && decl.ty == ctx.types.VkStructureType
        && !decl.metadata.values.is_empty()
    {
        code!(
            w,
            $import!(&ctx.types.VkStructureType)::$(decl.metadata.values.get(0).unwrap().resolve())
        )
    } else {
        let str = match overlay.resolve_alias(ctx).as_slice() {
            [TyToken::Ptr, TyToken::Const, ..] => "std::ptr::null()",
            [TyToken::Ptr, ..] => "std::ptr::null_mut()",
            [TyToken::Array(_), ..] => "unsafe { std::mem::zeroed() }",
            [TyToken::BaseType(basetype)] => {
                if is_function_pointer(*basetype, ctx) {
                    "None"
                } else {
                    "Default::default()"
                }
            }
            _ => panic!("Invalid type '{overlay}' for default initialization"),
        };
        w.write_str(str).unwrap();
    }
}
