use std::collections::HashSet;

use generator_lib::{
    configuration::GenConfig, interner::UniqueStr, Extension, Feature, FeatureExtensionItem,
    InterfaceItem, Registry, SymbolBody,
};

use crate::{is_std_type, resolve_alias, Context, Section, SectionKind, INVALID_SECTION};

// given that the registry is modelled as a global soup of types and functions from which the subsequent categories pick a subset
// we would like to split the definition into multiple files so that commit diffs aren't enormous, editor doesn't choke, and things are more organised
// we will just go through the categories (that are selected by the user and subsequent dependencies, so the included items may change around? TODO verify this)
// and the first (and mostly only one) that `requires` the items will also own them, if any
pub fn resolve_ownership(ctx: &mut Context) {
    // iterate through the sections and the items in their `require` tags
    // if the item doesn't have ownership, set it to the current section
    for (i, section) in ctx.sections.iter().enumerate() {
        for mut symbol in section_used_symbols(section, &ctx.conf, &ctx.reg) {
            if let Some((flags_name, _)) = ctx.flag_bits_to_flags(symbol) {
                symbol = flags_name;
            }

            if let Some(index) = ctx.get_symbol_index(symbol) {
                let entry = &mut ctx.symbol_ownership[index as usize];
                if *entry == INVALID_SECTION {
                    *entry = i as u32;
                }
            } else {
                // QUIRK
                //   types like uint8_t and such are removed from the registry as they are always replaced by references into the standard library
                //   video.xml puts the name of its header which obviously is not a symbol in the <require> tag of its extension
                if is_std_type(symbol, ctx) {
                    continue;
                }
                panic!("[{}] Unknown symbol.", symbol);
            }
        }
    }

    // validate ownerships
    for (i, item) in ctx.symbols.iter().enumerate() {
        // some BitmaskBits are left unowned if they contain no members :(
        if let SymbolBody::Enum { bitmask, .. } = item.1 {
            if bitmask {
                continue;
            }
        }

        if let SymbolBody::Alias(of) = item.1 {
            // std types are not in the registry, see workarounds.rs
            if is_std_type(of, &ctx) {
                break;
            }

            // aliases to BitmaskBits are allowed to be unowned
            if let &SymbolBody::Enum { bitmask, .. } = resolve_alias(of, ctx).1 {
                if bitmask {
                    continue;
                }
            }
        }

        match item.1 {
            SymbolBody::Included { .. } => unreachable!("[{}] Types included from headers are opaque and as such must be resolved by other means before this stage.", item.0.resolve()),
            _ => {
                assert!(
                    ctx.symbol_ownership[i] != INVALID_SECTION,
                    "[{}] Concrete types should have valid ownership assigned at this point.", item.0.resolve()

                )
            }
        }
    }
}

fn section_used_symbols<'a>(
    section: &'a Section,
    conf: &'a GenConfig,
    reg: &'a Registry,
) -> impl Iterator<Item = UniqueStr> + 'a {
    let children = match section.kind {
        SectionKind::Feature(idx) => {
            let Feature {
                name,
                api,
                protect,
                children,
                ..
            } = &reg.features[idx as usize];

            if skip_conf_conditions(api, None, *protect, Some(*name), None, conf) {
                None
            } else {
                Some(children.as_slice())
            }
        }
        SectionKind::Extension(idx) => {
            let Extension {
                name,
                protect,
                supported,
                children,
                ..
            } = &reg.extensions[idx as usize];

            if skip_conf_conditions(supported, Some(*name), *protect, None, None, conf) {
                None
            } else {
                Some(children.as_slice())
            }
        }
    };
    children
        .into_iter()
        .flat_map(|a| a.iter())
        .filter_map(|item| match item {
            FeatureExtensionItem::Comment(_) => None,
            FeatureExtensionItem::Require {
                items,
                profile,
                api,
                extension,
                feature,
            } => {
                if skip_conf_conditions(api, *extension, None, *feature, *profile, conf) {
                    None
                } else {
                    Some(items)
                }
            }
            FeatureExtensionItem::Remove { .. } => unimplemented!(),
        })
        .flat_map(move |a| {
            a.iter().filter_map(move |s| match s {
                InterfaceItem::Simple { name, .. } => Some(*name),
                // an owning section will not be extending its own enum, it will have defined the variants immediately
                InterfaceItem::Extend { .. } => None,
            })
        })
}

pub fn skip_conf_conditions(
    api: &[UniqueStr],
    extension: Option<UniqueStr>,
    protect: Option<UniqueStr>,
    feature: Option<UniqueStr>,
    profile: Option<UniqueStr>,
    conf: &GenConfig,
) -> bool {
    if multiple_match_skip(api, &conf.apis) {
        return true;
    }

    if let Some(extension) = extension {
        if !conf.is_extension_used(extension) {
            return true;
        }
    }

    if let Some(protect) = protect {
        if !conf.is_protect_used(protect) {
            return true;
        }
    }

    if let Some(feature) = feature {
        if feature != conf.feature {
            return true;
        }
    }

    if let Some(profile) = profile {
        if Some(profile) != conf.profile {
            return true;
        }
    }

    false
}

fn multiple_match_skip(extensions: &[UniqueStr], set: &HashSet<UniqueStr>) -> bool {
    if extensions.is_empty() {
        return false;
    }

    for extension in extensions {
        if set.contains(extension) {
            return false;
        }
    }

    true
}
