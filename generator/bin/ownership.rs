use generator_lib::{
    FeatureExtensionItem, InterfaceItem, Intern, ItemKind, Registry, Resolve, ToplevelBody,
};
use lasso::Spur;

use crate::{Context, Section, SectionKind, INVALID_SECTION};

// impl Ownership {
//     pub fn get_ownership(&self, item: &Spur, reg: &Registry) -> Option<u32> {
//         let section = *self.get_ownership_raw(item, reg);
//         if section == INVALID_SECTION {
//             None
//         } else {
//             Some(section)
//         }
//     }
//     pub fn get_ownership_raw(&self, item: &Spur, reg: &Registry) -> &u32 {
//         let &(index, kind) = reg.item_map.get(&item).expect("Item not in registry.");
//         assert_eq!(kind, ItemKind::Toplevel);
//         &self.items[index as usize]
//     }
//     pub fn get_section(&self, index: u32) -> &Section {
//         &self.sections[index as usize]
//     }
// }

// given that the registry is modelled as a global soup of types and functions from which the subsequent categories pick a subset
// we would like to split the definition into multiple files so that commit diffs aren't enormous, editor doesn't choke, and things are more organised
// we will just go through the categories (that are selected by the user and subsequent dependencies, so the included items may change around? TODO verify this)
// and the first (and mostly only one) that `requires` the items will also own them, if any
pub fn resolve_ownership(ctx: &mut Context /* , selected_sections: &mut Vec<Spur> */) {
    // corresponds to toplevel items, integer contained is index of the owning section
    let Context {
        reg,
        item_ownership,
        sections,
    } = ctx;

    for (i, item) in reg.toplevel.iter().enumerate() {
        match item.1 {
            ToplevelBody::Included { header } => {
                let entry = &mut item_ownership[i];
                if *entry != INVALID_SECTION {
                    panic!("Type can be included only once.")
                }

                // linear search seems fine as at this point only Header sections are in the vector of which there are not many
                let pos = sections.iter().position(|s: &Section| match s.kind {
                    SectionKind::Header(h) => h == header,
                    _ => false,
                });

                if let Some(pos) = pos {
                    *entry = pos as u32;
                } else {
                    *entry = sections.len() as u32;
                    sections.push(Section {
                        name: header,
                        kind: SectionKind::Header(header),
                    });
                }
            }
            _ => {}
        }
    }

    for (i, feature) in reg.features.iter().enumerate() {
        sections.push(Section {
            name: feature.name,
            kind: SectionKind::Feature(i as u32),
        });
    }

    for (i, extension) in reg.extensions.iter().enumerate() {
        sections.push(Section {
            name: extension.name,
            kind: SectionKind::Extension(i as u32),
        });
    }

    // iterate through the sections and the items in their `require` tags
    // if the item doesn't have ownership, set it to the current section
    for (i, section) in sections.iter().enumerate() {
        for item in section_used_items(section, reg) {
            let (index, kind) = reg.item_map.get(&item).expect("Item not in registry.");
            assert_eq!(*kind, ItemKind::Toplevel);

            let entry = &mut item_ownership[*index as usize];
            if *entry == INVALID_SECTION {
                *entry = i as u32;
            }
        }
    }

    // find the index of the `Vulkan 1` feature section, this is use in the next pass
    let vulkan1 = {
        let spur = "VK_VERSION_1_0".intern(reg);
        sections.iter().position(|s| s.name == spur).unwrap() as u32
    };

    // go through one last time and fix up missing ownerships
    for (i, item) in reg.toplevel.iter().enumerate() {
        match item.1 {
            ToplevelBody::Alias { .. } => {
                // sadly this may leave some ownership unset, in which case we shouldn't need to generate any code from that item
                // propagate_ownership(i as u32, &mut item_ownership, reg);
            }
            ToplevelBody::Constant { .. } => {
                // there are some global constants such as VK_MAX_PHYSICAL_DEVICE_NAME_SIZE
                // that are used as array sizes in type declarations and aren't in a `require` in the corresponding sections
                // we will just put them in the `Vulkan 1` section since they are so basic
                if item_ownership[i] == INVALID_SECTION {
                    item_ownership[i] = vulkan1;
                }
            }
            // some BitmaskBits are left unowned if they contain no members :(
            ToplevelBody::BitmaskBits { .. } => {}
            _ => {
                assert!(
                    item_ownership[i] != INVALID_SECTION,
                    "Concrete types should have valid ownership asigned at this point."
                )
            }
        }
    }
}

fn section_used_items<'a>(
    section: &'a Section,
    reg: &'a Registry,
) -> impl Iterator<Item = Spur> + 'a {
    let children = match section.kind {
        SectionKind::Header(_) => &[],
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
                    // vk_platform is in <require> tags even though it is a section .. why?
                    if *name == vk_platform {
                        None
                    } else {
                        Some(*name)
                    }
                }
                InterfaceItem::Extend { extends, .. } => Some(*extends),
                // this will always introduce a new local item, as such it is never in the toplevel items
                // furthermore this seem to only ever be extension versions and names which aren't required by anything else
                InterfaceItem::AddConstant { .. } => None,
            })
        })
}

// aliases may not receive ownership, in that case set them to that of their targets
fn propagate_ownership(alias_item_idx: u32, item_ownership: &mut [u32], reg: &Registry) -> u32 {
    let alias_item_idx = alias_item_idx as usize; // uuu

    if item_ownership[alias_item_idx] == INVALID_SECTION {
        match &reg.toplevel[alias_item_idx].1 {
            ToplevelBody::Alias { alias_of, .. } => {
                let &(next_idx, kind) = reg.item_map.get(alias_of).unwrap();
                assert_eq!(kind, ItemKind::Toplevel);
                item_ownership[alias_item_idx] = propagate_ownership(next_idx, item_ownership, reg);
            }
            // sadly some alias targets are never given ownership (see below)
            // we just return the invalid marker and structure later code to never try to acess its index if the bits are empty
            // since the marker is u32::MAX and would immediatelly fail a bound check
            // FIXME this is nonideal
            _ => return INVALID_SECTION,
        }
    }
    // have to keep reindexing like this to satisfy the borrow checker
    return item_ownership[alias_item_idx];
}

// call fn for every direct dependency of the section
fn foreach_dependency<F: FnMut(Section)>(section: &Section, mut f: F, reg: &Registry) {
    match section.kind {
        SectionKind::Header(_) => {} // no dependencies
        SectionKind::Feature(idx) => {
            let number = reg.features[idx as usize].number.resolve(reg);
            for (i, feature) in reg.features.iter().enumerate() {
                // we want to output all lower feature levels (essentially the vulkan version) than the one selected
                // thankfully this order is still valid on textual numbers
                // ie: "1.0" < "1.1" < "1.2"
                if &*feature.number.resolve(reg) < &*number {
                    f(Section {
                        name: feature.name,
                        kind: SectionKind::Feature(i as u32),
                    })
                }
            }
        }
        SectionKind::Extension(idx) => {
            let extension = &reg.extensions[idx as usize];
            for require in &extension.requires {
                let &(next_idx, kind) = reg.item_map.get(require).unwrap();
                let kind = match kind {
                    ItemKind::Feature => SectionKind::Feature(next_idx),
                    ItemKind::Extension => SectionKind::Extension(next_idx),
                    _ => unreachable!(),
                };
                f(Section {
                    name: *require,
                    kind,
                });
            }
            if let Some(core) = extension.requires_core {
                let index = reg.features.iter().position(|f| f.name == core).unwrap();
                foreach_dependency(
                    &Section {
                        name: reg.features[index].name,
                        kind: SectionKind::Feature(index as u32),
                    },
                    f,
                    reg,
                )
            }
        }
    }
}
