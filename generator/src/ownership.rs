use generator_lib::{
    interner::{Intern, UniqueStr},
    FeatureExtensionItem, InterfaceItem, ItemKind, Registry, SymbolBody,
};

use crate::{is_std_type, resolve_alias, Context, Section, SectionKind, INVALID_SECTION};

// given that the registry is modelled as a global soup of types and functions from which the subsequent categories pick a subset
// we would like to split the definition into multiple files so that commit diffs aren't enormous, editor doesn't choke, and things are more organised
// we will just go through the categories (that are selected by the user and subsequent dependencies, so the included items may change around? TODO verify this)
// and the first (and mostly only one) that `requires` the items will also own them, if any
pub fn resolve_ownership(ctx: &mut Context) {
    let Context {
        reg,
        symbol_ownership,
        sections,
        ..
    } = ctx;

    // iterate through the sections and the items in their `require` tags
    // if the item doesn't have ownership, set it to the current section
    for (i, section) in sections.iter().enumerate() {
        for symbol in section_used_symbols(section, reg) {
            if let Some(index) = reg.get_symbol_index(symbol) {
                let entry = &mut symbol_ownership[index as usize];
                if *entry == INVALID_SECTION {
                    *entry = i as u32;
                }
            } else {
                // QUIRK
                //   types like uint8_t and such are removed from the registry as they are always replaced by references into the standard library
                //   video.xml puts the name of its header which obviously is not a symbol in the <require> tag of its extension
            }
        }
    }

    // validate ownerships
    for (i, item) in reg.symbols.iter().enumerate() {
        if let SymbolBody::Alias(of) = item.1 {
            // std types are not in the registry, see workarounds.rs
            if is_std_type(of) {
                break;
            }
            // aliases to BitmaskBits are allowed to be unowned
            if let SymbolBody::BitmaskBits { .. } = resolve_alias(of, reg).1 {
                continue;
            }
        }

        match item.1 {
            // some BitmaskBits are left unowned if they contain no members :(
            SymbolBody::BitmaskBits { .. } => {}
            SymbolBody::Included { .. } => unreachable!("[{}] Types included from headers are opaque and as such must be resolved by other means before this stage.", item.0.resolve()
        ),
            _ => {
                assert!(
                    symbol_ownership[i] != INVALID_SECTION,
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
    let vk_platform = "vk_platform".intern(reg);
    children
        .iter()
        .filter_map(|item| match item {
            FeatureExtensionItem::Comment(_) => None,
            FeatureExtensionItem::Require { items, .. } => Some(items),
            FeatureExtensionItem::Remove { .. } => unimplemented!(),
        })
        .flat_map(move |a| {
            a.iter().filter_map(move |s| match s {
                InterfaceItem::Simple { name, .. } => {
                    // QUIRK vk_platform is in <require> tags even though it is a section .. why?
                    if *name == vk_platform {
                        None
                    } else {
                        Some(*name)
                    }
                }
                InterfaceItem::Extend { extends, .. } => Some(*extends),
            })
        })
}

// call fn for every direct dependency of the section
fn foreach_dependency<F: FnMut(Section)>(section: &Section, mut f: F, reg: &Registry) {
    match section.kind {
        SectionKind::Feature(idx) => {
            let number = reg.features[idx as usize].number.resolve();
            for (i, feature) in reg.features.iter().enumerate() {
                // we want to output all lower feature levels (essentially the vulkan version) than the one selected
                // thankfully this order is still valid on textual numbers
                // ie: "1.0" < "1.1" < "1.2"
                if &*feature.number.resolve() < &*number {
                    f(Section {
                        name: feature.name,
                        kind: SectionKind::Feature(i as u32),
                    })
                }
            }
        }
        SectionKind::Extension(idx) => {
            let extension = &reg.extensions[idx as usize];
            if let Some(core) = extension.requires_core {
                let index = reg.features.iter().position(|f| f.number == core).unwrap();
                f(Section {
                    name: reg.features[index].name,
                    kind: SectionKind::Feature(index as u32),
                });
            }
            for &name in &extension.requires {
                let &(index, kind) = reg.get_item_entry(name).unwrap();
                let kind = match kind {
                    ItemKind::Feature => SectionKind::Feature(index),
                    ItemKind::Extension => SectionKind::Extension(index),
                    _ => unreachable!(),
                };
                f(Section { name, kind });
            }
        }
    }
}

pub fn add_dependent_sections(selected_sections: &mut Vec<UniqueStr>, ctx: &Context) {
    let mut i = 0;
    while i < selected_sections.len() {
        let section = ctx.find_section(selected_sections[i]);
        foreach_dependency(
            section,
            |s| {
                if !selected_sections.contains(&s.name) {
                    selected_sections.push(s.name);
                }
            },
            &ctx.reg,
        );
        i += 1;
    }
}
