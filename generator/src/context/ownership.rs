use std::{collections::hash_map::Entry, rc::Rc};

use generator_lib::{
    interner::UniqueStr, DependsExpr, Extension, ExtensionKind, Feature, FeatureExtensionItem,
    InterfaceItem, Registry,
};

use crate::context::{Context, Section, SectionFunctions, SectionKind};

use super::SymbolMeta;

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
    for (section_handle, section) in sections.iter_kv() {
        let mut depends = None;
        section_used_symbols(section, reg, |event| {
            match event {
                UsedSymbolsEvent::OpenDepends(d) => {
                    depends = d.map(|d| Rc::new(d.clone()));
                }
                UsedSymbolsEvent::CloseDepends => {
                    depends = None;
                }
                UsedSymbolsEvent::Symbol(name) => {
                    if reg.get_symbol_index(name).is_some() {
                        match symbol_ownership.entry(name) {
                            Entry::Occupied(o) => {
                                let prev_section_handle = o.get().owner;
                                let prev_section = &sections[prev_section_handle];

                                // a symbol may be registered in the same section twice
                                // due to *FlagBits being renamed to *Flags
                                if section_handle != prev_section_handle {
                                    log::error!(
                                        "{name} in {} and {}",
                                        prev_section.name(),
                                        section.name()
                                    );
                                }
                            }
                            Entry::Vacant(v) => {
                                v.insert(SymbolMeta {
                                    owner: section_handle,
                                    depends: depends.clone(),
                                });
                            }
                        }
                    } else {
                        // QUIRK
                        //   workaround.rs may mark deleted symbols by renaming them to this placeholder
                        if name.eq_resolve(ctx.strings.__RESERVED_INVALID_PLACEHOLDER) {
                            return;
                        }

                        panic!("[{name}] Unknown symbol");
                    }
                }
            }
        })
    }

    // currently we validate that an item has valid ownership when we format them with `fmt_symbol_path`
    // this is neccessary as some symbols may end up unowned due to `skip_conf_conditions` skipping whole sections, yet they are then never referenced
    // it may be useful to re-add the code that was here previously which did early validation of some of the edge cases
}

enum UsedSymbolsEvent<'a> {
    OpenDepends(Option<&'a DependsExpr>),
    CloseDepends,
    Symbol(UniqueStr),
}

fn section_used_symbols(section: &Section, reg: &Registry, mut fun: impl FnMut(UsedSymbolsEvent)) {
    let children = match &section.kind {
        &SectionKind::Feature(idx) => {
            // {api, protect} have already been filtered while parsing the xml
            let Feature {
                name: _,
                api: _,
                protect: _,
                children,
                ..
            } = &reg.features[idx as usize];

            children
        }
        &SectionKind::Extension(idx) => {
            // {protect} has already been filtered while parsing the xml
            let Extension {
                name: _,
                depends: _,
                children,
                kind,
                ..
            } = &reg.extensions[idx as usize];

            if *kind == ExtensionKind::Disabled {
                return;
            }

            children
        }
        SectionKind::Path(_) => unimplemented!(),
    };

    for c in children {
        match c {
            FeatureExtensionItem::Comment(_) => {}
            // {api} has already been filtered while parsing the xml
            FeatureExtensionItem::Require {
                api: _,
                depends,
                items,
            } => {
                fun(UsedSymbolsEvent::OpenDepends(depends.as_ref()));

                for i in items {
                    match i {
                        InterfaceItem::Simple { name } => fun(UsedSymbolsEvent::Symbol(*name)),
                        // an owning section will not be extending its own enum, it will have defined the variants immediately
                        InterfaceItem::Extend { .. } => {}
                    }
                }

                fun(UsedSymbolsEvent::CloseDepends);
            }
            FeatureExtensionItem::Remove { .. } => unimplemented!(),
        }
    }
}
