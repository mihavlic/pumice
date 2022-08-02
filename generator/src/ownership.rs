use generator_lib::{
    interner::UniqueStr, FeatureExtensionItem, InterfaceItem, Registry, SymbolBody,
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
        for mut symbol in section_used_symbols(section, &ctx.reg) {
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
    reg: &'a Registry,
) -> impl Iterator<Item = UniqueStr> + 'a {
    let children = match section.kind {
        SectionKind::Feature(idx) => reg.features[idx as usize].children.as_slice(),
        SectionKind::Extension(idx) => reg.extensions[idx as usize].children.as_slice(),
    };
    children
        .iter()
        .filter_map(|item| match item {
            FeatureExtensionItem::Comment(_) => None,
            FeatureExtensionItem::Require { items, .. } => Some(items),
            FeatureExtensionItem::Remove { .. } => unimplemented!(),
        })
        .flat_map(move |a| {
            a.iter().filter_map(move |s| match s {
                InterfaceItem::Simple { name, .. } => Some(*name),
                // an owning section will not be extending its own enum, it will have defined the variants imediatelly
                InterfaceItem::Extend { .. } => None,
            })
        })
}
