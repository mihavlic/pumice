use std::collections::HashMap;

use generator_lib::{foreach_uniquestr::ForeachUniquestr, interner::UniqueStr, Symbol, SymbolBody};

use crate::{is_std_type, Context};

pub fn undangle(ctx: &mut Context) {
    let mut map: HashMap<UniqueStr, UniqueStr> = HashMap::new();
    for &(symbol_idx, _) in &ctx.symbols {
        if let Symbol(name, SymbolBody::Alias(of)) = &mut ctx.reg.symbols[symbol_idx] {
            if let Some(to) = map.get(of) {
                *of = *to;
                continue;
            }

            let name = *name;
            let of = *of;
            if ctx.symbol_get_section_idx(of).is_none() {
                if is_std_type(of, ctx) {
                    continue;
                }

                let target =
                    ctx.get_symbol_index(of)
                        .unwrap_or_else(|| panic!("{} {}", name, of)) as usize;

                // asert that this is not an alias of itself and its meory doesn't alias
                assert!(symbol_idx != target);
                let ptr1: *mut _ = &mut ctx.reg.symbols[symbol_idx].1;
                let ptr2: *mut _ = &mut ctx.reg.symbols[target].1;
                unsafe {
                    // we are overwriting a SymbolBOdy::Alias which is pod so we can afford not to drop it
                    std::ptr::write(ptr1, std::ptr::read(ptr2));
                    // some symbols may still refer to the replaced definition
                    // so we point it to the one which is now "oficial"
                    std::ptr::write(ptr2, SymbolBody::Alias(name));
                }

                map.insert(of, name);
            }
        }
    }

    for &(symbol_idx, _) in &ctx.symbols {
        ctx.reg.symbols[symbol_idx].foreach(&mut |str| {
            if let Some(to) = map.get(str) {
                *str = *to;
            }
        });
    }
}
