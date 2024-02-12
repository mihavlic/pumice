pub mod format_utils;
pub mod rename;
pub mod type_analysis;
pub mod type_query;

use crate::{codegen_support::type_analysis::get_underlying_type, context::Context};
use generator_lib::{
    interner::{Intern, UniqueStr},
    ConstantValue, Declaration, DependsExpr, ExtensionKind, FeatureExtensionItem, InterfaceItem,
};
use std::{collections::HashMap, rc::Rc};

pub enum CommandKind {
    Entry,
    Instance,
    Device,
}

// essentially allows matching against runtime values
#[macro_export]
macro_rules! switch {
    ($what:expr; $( $($e:expr),+ => $to:expr;)+ @ $esle:expr) => {
        $(
            if $($what == $e) ||+  {
                $to
            } else
        )+
        {
            $esle
        }
    };
}

pub fn get_command_kind(params: &[Declaration], ctx: &Context) -> CommandKind {
    // idea shamelessly stolen from erupt
    if let Some(param) = params.get(0) {
        if let Some(basetype) = param.ty.try_only_basetype() {
            let s = &ctx.strings;
            switch!( basetype;
                s.VkInstance, s.VkPhysicalDevice => return CommandKind::Instance;
                s.VkDevice, s.VkCommandBuffer, s.VkQueue => return CommandKind::Device;
                @ {}
            )
        }
    }
    CommandKind::Entry
}

#[derive(Debug)]
pub struct AddedVariants<'a> {
    pub source_section: UniqueStr,
    pub name: UniqueStr,
    pub value: &'a ConstantValue,
    pub depends: Option<Rc<DependsExpr>>,
}

pub fn get_enum_added_variants(ctx: &Context) -> HashMap<UniqueStr, Vec<AddedVariants<'_>>> {
    let mut enum_supplements: HashMap<UniqueStr, Vec<AddedVariants<'_>>> = HashMap::new();

    let features = ctx.reg.features.iter().map(|f| (f.name, &f.children));
    let extensions = ctx
        .reg
        .extensions
        .iter()
        .filter(|e| e.kind != ExtensionKind::Disabled)
        .map(|e| (e.name, &e.children));

    for (section_name, children) in features.chain(extensions) {
        for item in children {
            match item {
                FeatureExtensionItem::Comment(_) => {}
                FeatureExtensionItem::Require {
                    api: _,
                    depends,
                    items,
                } => {
                    let depends = depends.as_ref().map(|d| Rc::new(d.clone()));
                    for item in items {
                        if let &InterfaceItem::Extend {
                            name,
                            extends,
                            ref value,
                        } = item
                        {
                            let variant = AddedVariants {
                                source_section: section_name,
                                name,
                                value,
                                depends: depends.clone(),
                            };
                            enum_supplements.entry(extends).or_default().push(variant);
                        }
                    }
                }
                FeatureExtensionItem::Remove { .. } => unimplemented!(),
            }
        }
    }
    enum_supplements
}

pub fn merge_bitfield_members<'a>(members: &'a [Declaration], ctx: &Context) -> Vec<Declaration> {
    let mut resolved: Vec<Declaration> = Vec::new();
    let mut last_decl: Option<&Declaration> = None;
    let mut current_bits = 0;
    let mut max_bits = 0;
    let mut merged_members: Vec<UniqueStr> = Vec::new();

    for member in members {
        let Declaration {
            name, ty, bitfield, ..
        } = member;

        // the type matches and it is a bitfield
        if Some(ty) == last_decl.map(|d| &d.ty) && bitfield.is_some() {
            let bits = bitfield.unwrap();
            assert!(bits <= max_bits);
            current_bits += bits;
            // we still have space to merge this member
            if current_bits <= max_bits {
                merged_members.push(*name);
                continue;
            }
            // otherwise we just pass through and the merged members are picked up and merged
            // and the current member is added to the next batch
        };

        // merge the accumulated members into one member that will have to be packed and unpacked by the user
        if let Some(decl) = last_decl.take() {
            assert!(!merged_members.is_empty());
            let mut decl = decl.to_owned();

            // TODO consider some better naming rather than just concatenating everything
            let name = if merged_members.len() == 1 {
                merged_members[0]
            } else {
                let mut concat = merged_members[0].resolve().to_owned();
                for member in &merged_members[1..] {
                    concat += "_";
                    concat += member.resolve();
                }
                concat.intern(ctx)
            };
            decl.bitfield = Some(current_bits);
            decl.name = name;

            resolved.push(decl);
            merged_members.clear();
        }

        // start accumulating a new type, if it isn't a bitfield, we add it to the resolved vec straight away,
        // since last_ty is still None, the next member that comes skips both of the block above and can either
        // start accumulating because it is a bitfield or is again just passed through to resolved
        if let Some(bits) = bitfield {
            // microsoft (https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170) says that only primitive types
            // can be bitfields, in practice this means that the type tokens will be just an ident
            let basetype = ty
                .try_only_basetype()
                .expect("Only a base raw integer can be a bitfield.");

            let s = &ctx.strings;
            let underlying = get_underlying_type(basetype, ctx).get_basetype();
            let type_bits = switch!(underlying;
                s.uint8_t,  s.int8_t => 8;
                s.uint16_t, s.int16_t => 16;
                s.uint32_t, s.int32_t => 32;
                s.uint64_t, s.int64_t => 64;
                @ unimplemented!("Don't know the bit-width of '{}'", underlying)
            );

            assert!(*bits <= type_bits);

            max_bits = type_bits;
            current_bits = *bits;
            last_decl = Some(&member);
            merged_members.push(*name);
        } else {
            resolved.push(member.clone());
        }
    }

    resolved
}
