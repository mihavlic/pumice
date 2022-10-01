pub mod format_utils;
pub mod rename;
pub mod type_analysis;
pub mod type_query;

use crate::{
    codegen_support::type_analysis::get_underlying_type,
    context::{ownership::skip_conf_conditions, Context},
};
use generator_lib::{
    interner::{Intern, UniqueStr},
    ConstantValue, Declaration, ExtensionKind, FeatureExtensionItem, InterfaceItem,
};
use std::collections::HashMap;

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

pub struct AddedVariants<'a> {
    pub source_section: UniqueStr,
    pub applicable: bool,
    pub variants: Vec<(UniqueStr, &'a ConstantValue)>,
}

pub fn get_enum_added_variants(ctx: &Context) -> HashMap<UniqueStr, Vec<AddedVariants<'_>>> {
    let mut enum_supplements: HashMap<UniqueStr, Vec<AddedVariants<'_>>> = HashMap::new();

    let features = ctx
        .reg
        .features
        .iter()
        .map(|f| (ctx.conf.is_feature_used(f.name), f.name, &f.children));
    let extensions = ctx
        .reg
        .extensions
        .iter()
        .filter(|e| e.kind != ExtensionKind::Disabled)
        .map(|e| {
            (
                ctx.conf.is_extension_used(e.name)
                // when an extention is promoted to core, all its enum values are copied into a
                // feature <require> thus variants from the extensions are no longer applicable if
                // this occurs note that this isn't done for constants because extensions specify
                // their name and version that way
                && !e.promotedto
                    .map(|core| ctx.conf.is_feature_used(core))
                    .unwrap_or(false),
                e.name,
                &e.children,
            )
        });

    for (applicable, section_name, children) in features.chain(extensions) {
        for item in children {
            match item {
                FeatureExtensionItem::Comment(_) => {}
                FeatureExtensionItem::Require {
                    profile,
                    api,
                    extension,
                    feature,
                    items,
                } => {
                    let applicable = applicable
                        && !skip_conf_conditions(
                            api, *extension, None, *feature, *profile, &ctx.conf,
                        );
                    'outer: for item in items {
                        match item {
                            InterfaceItem::Simple { .. } => {}
                            &InterfaceItem::Extend {
                                name,
                                extends,
                                ref value,
                            } => {
                                let entry = enum_supplements.entry(extends).or_insert(Vec::new());

                                let add = (name, value);

                                // we deduplicate the variants here, because khronos was so nice to willingly put
                                // duplicates in the registry, like VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR
                                'middle: for added in &mut *entry {
                                    for i in 0..added.variants.len() {
                                        let &(n, _) = &added.variants[i];
                                        if n == name {
                                            if !applicable {
                                                continue 'outer;
                                            }
                                            // if the added variant is not applicable, ie. soft-deleted
                                            // we remove it and overwrite it with the current one, otherwise we skip this one
                                            else if !added.applicable {
                                                added.variants.remove(i);
                                                break 'middle;
                                            } else {
                                                continue;
                                            }
                                        }
                                    }
                                }

                                if let Some(a) =
                                    entry.iter_mut().find(|a| a.source_section == section_name)
                                {
                                    a.variants.push(add);
                                } else {
                                    entry.push(AddedVariants {
                                        source_section: section_name,
                                        applicable: applicable,
                                        variants: vec![add],
                                    });
                                }
                            }
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
            let underlying = get_underlying_type(basetype, ctx).try_basetype().unwrap();
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
