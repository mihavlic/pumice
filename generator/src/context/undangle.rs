use std::collections::HashMap;

use generator_lib::{foreach_uniquestr::ForeachUniquestr, interner::UniqueStr, Symbol, SymbolBody};

use super::Context;

pub fn undangle(ctx: &mut Context) {
    let mut map: HashMap<UniqueStr, UniqueStr> = HashMap::new();
    for &(symbol_idx, _) in &ctx.used_symbols {
        if let Symbol(name, SymbolBody::Alias(alias)) = &mut ctx.reg.symbols[symbol_idx] {
            if let Some(to) = map.get(alias) {
                *alias = *to;
                continue;
            }

            let name = *name;
            let alias = *alias;
            if ctx.get_symbol_section(alias).is_none() {
                let target = ctx
                    .get_symbol_index(alias)
                    .unwrap_or_else(|| panic!("{alias}, alias of {name} is not a symbol"))
                    as usize;

                // asert that this is not an alias of itself
                assert!(symbol_idx != target);

                let ptr1: *mut _ = &mut ctx.reg.symbols[symbol_idx].1;
                let ptr2: *mut _ = &mut ctx.reg.symbols[target].1;

                unsafe {
                    // we are overwriting a SymbolBody::Alias which is pod so we can afford not to drop it
                    std::ptr::write(ptr1, std::ptr::read(ptr2));
                    // some symbols may still refer to the replaced definition
                    // so we point it to the one which is now "oficial"
                    std::ptr::write(ptr2, SymbolBody::Alias(name));
                }

                map.insert(alias, name);
            }
        }
    }

    for &(symbol_idx, _) in &ctx.used_symbols {
        ctx.reg.symbols[symbol_idx].foreach(&mut |str| {
            if let Some(to) = map.get(str) {
                *str = *to;
            }
        });
    }
}
