use codewrite::{CFmt, Cond, Fun, FunOnce, Iter};
use codewrite_macro::code;
use generator_lib::{
    interner::{Intern, UniqueStr},
    type_declaration::{Token, TokenIter, TyToken, Type, TypeRef},
    Declaration, Optional, SymbolBody,
};

use std::{cell::RefCell, fmt::Write};

use crate::{
    cat,
    codegen::symbols::{fmt_default_value, fmt_default_value_with_overlay},
    codegen_support::{format_utils::SectionWriter, type_analysis::TypeAnalysis, CommandKind},
    context::Context,
    doc_boilerplate, import,
};

/// All of these algorithms are adapted straight from Erupt

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParameterRole<'a> {
    None,
    Regular,
    Array,
    Optional,
    Passthrough,
    // function takes a dispatchable handle and works as an associated function
    Handle,
    CStr,
    ValueWrittenTo {
        inner: &'a TypeRef,
    },
    ArrayWrittenTo {
        inner: &'a TypeRef,
        length: UniqueStr,
    },
    LengthByArray {
        array_indices: Vec<usize>,
    },
    // serves as a length retrieval and then length specifying pointer
    // over two subsequent call to the function
    LengthByExtraCall {
        array_indices: Vec<usize>,
        inner: &'a TypeRef,
    },
}

fn generate_list<'a>(
    parameters: &'a [Declaration],
    kind: CommandKind,
    ctx: &Context,
) -> Vec<ParameterRole<'a>> {
    let handle_type = get_handle_type(kind, ctx);

    let mut kinds = vec![ParameterRole::None; parameters.len()];
    let mut passthrough = Vec::new();

    for group in 0.. {
        // Finish algorithm if every kind is filled
        if kinds.iter().all(|a| a != &ParameterRole::None) {
            break;
        }

        let params_iter = parameters.iter().zip(kinds.iter_mut()).enumerate();
        for (i, (param, param_kind)) in params_iter {
            // Don't test for this parameter if it already has a kind
            if *param_kind != ParameterRole::None {
                continue;
            }

            let void_ptr = param.ty.is_void_pointer(ctx);

            match group {
                // Apply `Handle` kind if it qualifies as a handle
                0 => {
                    if i == 0 && Some(&param.ty) == handle_type.as_ref() {
                        *param_kind = ParameterRole::Handle;
                    }
                }
                // Apply `CStr` kind if the parameter is a char pointer
                1 => {
                    if param.ty == ctx.types.cstring {
                        *param_kind = ParameterRole::CStr;
                    }
                }
                // Mark the length passthrough if this parameter is passthrough
                2 => {
                    if void_ptr {
                        if let Some(&length) = param.metadata.length.get(0) {
                            passthrough.push(length);
                        }
                    }
                }
                // Apply `Passthrough` kind if applicable
                3 => {
                    if void_ptr || passthrough.contains(&param.name) {
                        *param_kind = ParameterRole::Passthrough;
                    }
                    // our handles are the same as in vulkan, thus just a u64
                    // optionality is handled by the Default methods and such
                    else if let Some(basetype) = param.ty.try_only_basetype() {
                        if matches!(ctx.get_symbol(basetype), Some(SymbolBody::Handle { .. })) {
                            *param_kind = ParameterRole::Passthrough;
                        }
                    }
                }
                // Apply length kinds
                4 => {
                    if let Some(array_indices) = get_indexes_of_length_targets(param, parameters) {
                        let opt = (
                            &param.metadata.optional,
                            param.ty.0.as_slice(),
                            param.metadata.length.get(0),
                        );
                        *param_kind = match opt {
                            (Optional::Sometimes, &[TyToken::Ptr, tok, ..], None)
                                if tok != TyToken::Const =>
                            {
                                ParameterRole::LengthByExtraCall {
                                    array_indices,
                                    inner: param.ty.descend(),
                                }
                            }
                            _ => ParameterRole::LengthByArray { array_indices },
                        };
                    }
                }
                // Apply other kinds
                5 => {
                    let var_name = (
                        &param.metadata.optional,
                        &param.ty.0.as_slice(),
                        &param.metadata.length.get(0),
                    );
                    match var_name {
                        (Optional::Never, &[TyToken::Ptr, tok, ..], None)
                            if *tok != TyToken::Const =>
                        {
                            let to = param.ty.descend();
                            *param_kind = ParameterRole::ValueWrittenTo { inner: to };

                            // Why was this here?
                            // if let Some(basetype) = to.try_only_basetype() {
                            //     if let SymbolBody::Struct { members, .. } =
                            //         ctx.get_symbol(basetype).unwrap()
                            //     {
                            //         if contains_pnext(members, ctx) {
                            //             *param_kind = ParameterRole::Regular;
                            //         }
                            //     }
                            // }
                        }
                        (
                            Optional::Never | Optional::Always,
                            &[TyToken::Ptr, tok, ..],
                            Some(length),
                        ) if *tok != TyToken::Const => {
                            *param_kind = ParameterRole::ArrayWrittenTo {
                                inner: param.ty.descend(),
                                length: **length,
                            };
                        }
                        (Optional::Always, _, None) => *param_kind = ParameterRole::Optional,
                        (_, _, Some(_)) => *param_kind = ParameterRole::Array,
                        (_, _, None) => *param_kind = ParameterRole::Regular,
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    kinds
}

pub fn get_handle_type(kind: CommandKind, ctx: &Context) -> Option<Type> {
    match kind {
        CommandKind::Entry => None,
        CommandKind::Instance => Some(Type::from_only_basetype(ctx.strings.VkInstance)),
        CommandKind::Device => Some(Type::from_only_basetype(ctx.strings.VkDevice)),
    }
}

pub fn get_indexes_of_length_targets(
    decl: &Declaration,
    params: &[Declaration],
) -> Option<Vec<usize>> {
    let array_indices: Vec<_> = params
        .iter()
        .enumerate()
        .filter(|(_, array)| array.metadata.length.get(0) == Some(&decl.name))
        .map(|(i, _)| i)
        .collect();

    (!array_indices.is_empty()).then(|| array_indices)
}

pub fn contains_pnext(members: &[Declaration], ctx: &Context) -> bool {
    members
        .iter()
        .find(|m| m.name == ctx.strings.pNext && m.ty == ctx.types.void_ptr)
        .is_some()
}

pub fn fmt_command_wrapper(
    w: &mut SectionWriter,
    function_name: UniqueStr,
    body: &SymbolBody,
    kind: CommandKind,
    ctx: &Context,
) {
    #[derive(Clone, Copy, PartialEq)]
    enum What {
        Params,
        Return,
        ReturnInit,
        LengthInit,
        Call,
        ReturnValues,
    }

    struct ExtraCallInfo<'a> {
        array_indices: &'a Vec<usize>,
        length_index: usize,
        param: &'a Declaration,
        name: &'a str,
        inner: &'a TypeRef,
    }

    if let SymbolBody::Command {
        success_codes,
        error_codes: _,
        params,
        return_type,
    } = body
    {
        let kinds = generate_list(params, kind, ctx);

        let extra_call_info = RefCell::new(None);

        fn write_param_analog<'a>(
            w: &mut SectionWriter,
            i: usize,
            params: &'a [Declaration],
            kinds: &'a [ParameterRole],
            extra_call_info: &RefCell<Option<ExtraCallInfo<'a>>>,
            state: What,
            ctx: &Context,
        ) {
            let param = &params[i];
            let kind = &kinds[i];
            let name = strip_pointer_stuff(param.name.resolve());
            match kind {
                ParameterRole::Regular => match state {
                    What::Params => {
                        let mut ty = &param.ty;
                        // convert VkBool32 (u32) into a native bool in the arguments
                        // which is then converted to the integer with a `bool as _`
                        // TODO move into a function
                        if ty == &ctx.types.VkBool32 {
                            ty = &ctx.types.bool;
                        }
                        let ty = ty.ptr_to_ref();
                        code!(w, $name: $import!(ty.as_ref()),);
                    }
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => code!(w, $name as _,),
                    What::ReturnValues => {}
                },
                ParameterRole::Array => match state {
                    What::Params => {
                        let ty = param.ty.strip_indirection();
                        code!(w, $name: &[$import!(ty)],)
                    }
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => code!(w, $name.as_ptr(),),
                    What::ReturnValues => {}
                },
                ParameterRole::Optional => match state {
                    What::Params => {
                        let ty = param.ty.ptr_to_ref();
                        code!(w, $name: Option<$import!(ty.as_ref())>,)
                    }
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => {
                        let default = Fun::new(|w: &mut SectionWriter| {
                            fmt_default_value(w, &param, ctx);
                        });
                        code!(
                            w,
                            match $name {
                                Some(v) => v,
                                None => $default,
                            },
                        )
                    }
                    What::ReturnValues => {}
                },
                ParameterRole::Passthrough => match state {
                    What::Params => code!(w, $name: $import!(&param.ty),),
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => code!(w, $name,),
                    What::ReturnValues => {}
                },
                ParameterRole::Handle => match state {
                    What::Params => {}
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => code!(w, self.handle,),
                    What::ReturnValues => {}
                },
                ParameterRole::CStr => match state {
                    What::Params => code!(w, $name: Option<&std::ffi::CStr>,),
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {}
                    What::Call => {
                        let default = Fun::new(|w: &mut SectionWriter| {
                            fmt_default_value(w, &param, ctx);
                        });
                        code!(
                            w,
                            match $name {
                                Some(v) => v.as_ptr(),
                                None => $default,
                            },
                        )
                    }
                    What::ReturnValues => {}
                },
                &ParameterRole::ValueWrittenTo { inner } => match state {
                    What::Params => {}
                    What::Return => code!(w, $import!(inner),),
                    What::ReturnInit => {
                        let default = Fun::new(|w: &mut SectionWriter| {
                            fmt_default_value_with_overlay(w, &param, inner, ctx);
                        });
                        code!(
                            w,
                            let mut $name = $default;
                        );
                    }
                    What::LengthInit => {}
                    What::Call => code!(w, &mut $name,),
                    What::ReturnValues => code!(w, $name,),
                },
                &ParameterRole::ArrayWrittenTo { inner, length } => {
                    let default = Fun::new(|w: &mut SectionWriter| {
                        fmt_default_value_with_overlay(w, &param, inner, ctx);
                    });
                    let len_path = len_paths(
                        length.resolve(),
                        |_| VarSource::InScope,
                        |s| strip_pointer_stuff(s),
                        ".",
                        ctx,
                    );

                    match state {
                        What::Return => code!(w, Vec<$import!(inner)>,),
                        What::Params | What::ReturnInit => {
                            if state == What::ReturnInit {
                                code!(w, let mut $name = vec![$default; $len_path as usize];)
                            }

                            if let Some(basetype) = inner.try_only_basetype() {
                                if let Some(SymbolBody::Struct { members, .. }) =
                                    ctx.get_symbol(basetype)
                                {
                                    if contains_pnext(members, ctx) {
                                        let callback_name = cat!(name, "_callback");

                                        if state == What::Params {
                                            code!(
                                                w,
                                                mut $callback_name: impl FnMut(&mut Vec<$import!(inner)>),
                                            )
                                        }

                                        if state == What::ReturnInit {
                                            code!(
                                                w,
                                                $callback_name(&mut $name);
                                            )
                                        }
                                    }
                                }
                            }
                        }
                        What::LengthInit => {}
                        What::Call => code!(w, $name.as_mut_ptr(),),
                        What::ReturnValues => code!(w, $name,),
                    }
                }
                ParameterRole::LengthByArray { array_indices } => match state {
                    What::Params => {}
                    What::Return => {}
                    What::ReturnInit => {}
                    What::LengthInit => {
                        let length_exprs = Fun::new(|w: &mut SectionWriter| {
                            array_indices
                                .into_iter()
                                .map(|&array_index| &params[array_index])
                                .filter(|array| match &array.ty.as_slice() {
                                    &[TyToken::Ptr, TyToken::Const, ..] => true,
                                    _ => false,
                                })
                                .enumerate()
                                .for_each(|(i, array)| {
                                    let ident = strip_pointer_stuff(array.name.resolve());
                                    match i {
                                        0 => code!(w, $ident.len() ),
                                        _ => code!(w, .min($ident.len()) ),
                                    }
                                });
                        });

                        code!(
                            w,
                            let $name = $length_exprs;
                        )
                    }
                    What::Call => code!(w, $name as _,),
                    What::ReturnValues => {}
                },
                ParameterRole::LengthByExtraCall {
                    array_indices,
                    inner,
                } => {
                    if extra_call_info.borrow().is_none() {
                        *extra_call_info.borrow_mut() = Some(ExtraCallInfo {
                            array_indices,
                            length_index: i,
                            param,
                            name,
                            inner: *inner,
                        });
                    }

                    match state {
                        What::Params => code!(w, $name: Option<$import!(*inner)>,),
                        What::Return => {}
                        What::ReturnInit => {}
                        What::LengthInit => {}
                        What::Call => code!(w, &mut $name,),
                        What::ReturnValues => {}
                    }
                }
                ParameterRole::None => unreachable!(),
            }
        }

        let return_type_count = kinds
            .iter()
            .filter(|k| {
                matches!(
                    k,
                    ParameterRole::ValueWrittenTo { .. } | ParameterRole::ArrayWrittenTo { .. }
                )
            })
            .count();

        let result = return_type == &ctx.types.VkResult;
        let parens = return_type_count == 0 || return_type_count > 1;
        let only_success = success_codes.as_slice() == &[ctx.strings.VK_SUCCESS];

        let function_arguments = Iter::new(0..params.len(), |w, i| {
            write_param_analog(w, i, params, &kinds, &extra_call_info, What::Params, ctx)
        });
        let function_call = Iter::new(0..params.len(), |w, i| {
            write_param_analog(w, i, params, &kinds, &extra_call_info, What::Call, ctx);
        });

        let function_return = Fun::new(|w: &mut SectionWriter| {
            if !result && return_type_count == 0 {
                return;
            }

            let inner = Fun::new(|w: &mut SectionWriter| {
                if parens {
                    w.write_char('(').unwrap();
                }
                {
                    (0..params.len()).for_each(|i| {
                        write_param_analog(
                            w,
                            i,
                            params,
                            &kinds,
                            &extra_call_info,
                            What::Return,
                            ctx,
                        );
                    });

                    if Some(',') == w.last_character() {
                        w.pop_last_character();
                    }
                }
                if parens {
                    w.write_char(')').unwrap();
                }
            });

            if result {
                // VulkanResult is not vkResult but the normal error enum

                // the only success condition is VK_SUCCESS, in this case we do not return the result in the Ok variant
                if only_success {
                    code!(w, -> $import!(&ctx.types.VulkanResult)<$inner>);
                } else {
                    let result_enum = import!(&ctx.types.VkResult);
                    if return_type_count == 0 {
                        code!(w, -> $import!(&ctx.types.VulkanResult)<$result_enum>);
                    } else {
                        code!(w, -> $import!(&ctx.types.VulkanResult)<($inner, $result_enum)>);
                    }
                }
            } else {
                code!(w, -> $inner);
            }
        });
        let extra_call = Fun::new(|w: &mut SectionWriter| {
            extra_call_info.borrow().as_ref().map(|info| {
                let default = Fun::new(|w: &mut SectionWriter| {
                    fmt_default_value_with_overlay(w, info.param, info.inner, ctx);
                });

                let length_index = info.length_index;
                let array_indices = info.array_indices;
                let name = info.name;

                let call_args = Iter::new(0..params.len(), |w: &mut SectionWriter, i| {
                    if i == length_index {
                        code!(w, &mut v,);
                    } else if array_indices.contains(&i) {
                        code!(w, std::ptr::null_mut(),);
                    } else {
                        write_param_analog(w, i, params, &kinds, &extra_call_info, What::Call, ctx);
                    }
                });

                code!(
                    w,
                    let mut $name = match $name {
                        Some(v) => v,
                        None => {
                            let mut v = $default;
                            $function_name($call_args);
                            v
                        }
                    };
                );
            });
        });
        let length_init = Fun::new(|w: &mut SectionWriter| {
            (0..params.len()).for_each(|i| {
                write_param_analog(
                    w,
                    i,
                    params,
                    &kinds,
                    &extra_call_info,
                    What::LengthInit,
                    ctx,
                );
            });
        });
        let return_init = Fun::new(|w: &mut SectionWriter| {
            (0..params.len()).for_each(|i| {
                write_param_analog(
                    w,
                    i,
                    params,
                    &kinds,
                    &extra_call_info,
                    What::ReturnInit,
                    ctx,
                );
            });
        });
        let return_expr = Fun::new(|w: &mut SectionWriter| {
            if !result && return_type_count == 0 {
                return;
            }

            let inner = Fun::new(|w: &mut SectionWriter| {
                if parens {
                    w.write_char('(').unwrap();
                }
                {
                    (0..params.len()).for_each(|i| {
                        write_param_analog(
                            w,
                            i,
                            params,
                            &kinds,
                            &extra_call_info,
                            What::ReturnValues,
                            ctx,
                        );
                    });
                }

                if Some(',') == w.last_character() {
                    w.pop_last_character();
                }

                if parens {
                    w.write_char(')').unwrap();
                }
            });

            if result {
                let value = Fun::new(|w: &mut SectionWriter| {
                    if only_success {
                        code!(w, $inner);
                    } else {
                        if return_type_count == 0 {
                            code!(w, result);
                        } else {
                            code!(w, ($inner, result));
                        }
                    }
                });

                code!(
                    w,
                    crate::new_result($value, result)
                );
            } else {
                code!(w, $inner)
            }
        });

        code!(
            w,
            #[track_caller]
            $(Cond::new(result, "#[must_use]"))
            $doc_boilerplate!(function_name)
            pub unsafe fn $function_name(&self, $function_arguments) $function_return {
                let $function_name = (*self.table).$function_name.unwrap();
                $extra_call
                $length_init
                $return_init
                $(Cond::new(result, "let result =")) $function_name($function_call);
                $return_expr
            }
        );
    } else {
        unreachable!()
    }
}

/// Eat all the p's at the start of the str and an underscore.
/// Otherwise return the whole string
fn strip_pointer_stuff(str: &str) -> &str {
    let mut chars = str.chars();
    let mut isp = false;

    while let Some(next) = chars.next() {
        match next {
            'p' => isp = true,
            '_' if isp => return chars.as_str(),
            _ => break,
        }
    }

    return str;
}

#[derive(Clone, Copy, PartialEq)]
pub enum VarSource {
    InScope,
    InSelf,
    Import,
}

pub fn len_paths<'a>(
    str: &'a str,
    get_source: impl Fn(UniqueStr) -> VarSource + 'a,
    on_ident: impl Fn(&str) -> &str + 'a,
    deref_acess: &'a str,
    ctx: &'a Context,
) -> impl CFmt<SectionWriter> + 'a {
    let mut state = 0;
    FunOnce::new(move |w: &mut SectionWriter| {
        for token in TokenIter::new(str) {
            let str = match token {
                Token::Number(num) => {
                    code!(w, $num);
                    continue;
                }
                Token::Ident(str) => match state {
                    0 => {
                        state = 1;
                        let uniq = str.intern(ctx);
                        match get_source(uniq) {
                            VarSource::InScope => code!(w, $(on_ident(uniq.resolve()))),
                            VarSource::InSelf => code!(w, self.$(on_ident(uniq.resolve()))),
                            VarSource::Import => code!(w, $import!(uniq)),
                        }
                        continue;
                    }
                    1 => {
                        let renamed = ctx.reg.lookup_rename(str);
                        code!(w, .$(on_ident(renamed)));
                        continue;
                    }
                    _ => unreachable!(),
                },
                Token::Struct => panic!("Token '{:?}' cannot occur in a path!", token),
                Token::Const => panic!("Token '{:?}' cannot occur in a path!", token),
                Token::LBracket => "[",
                Token::RBracket => "]",
                Token::LParen => "(",
                Token::RParen => ")",
                Token::LBrace => "{",
                Token::RBrace => "}",
                Token::DoubleColon => ":",
                Token::Tilde => "~",
                Token::Exclamation => "!",
                Token::Percent => "%",
                Token::Caret => "^",
                Token::Ampersand => "&",
                Token::Star => "*",
                Token::Slash => "/",
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Dot => ".",
                // since this is rust we don't have a concise way to dereference AND accessing a pointer
                Token::Arrow => deref_acess,
                Token::Unknown(c) => panic!("Unknown character '{c}' in '{str}'"),
                Token::End => unreachable!(),
            };
            state = 0;
            code!(w, $str);
        }
    })
}
