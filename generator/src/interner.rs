use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    mem,
};

use quote::{
    ToTokens,
    __private::{Ident, Span},
    quote,
};

// somewhat naively interns strings by copying them to chunks of memory which are held alive
// for the whole lifetime of the program, effectively leaking them
//
// u32 packed with 4 bits of block index and 28 bits of offset into the block, this can do about 1G of memory which should be sufficient for strings
// code for verification (assumes 16 bytes initial allocation)
// fn main() {
//     let block_bits = 4;
//     let memory_bits = 32-block_bits;

//     let mut alloc: u32 = 16;
//     let mut blocks = 1;
//     loop {
//         alloc = alloc * 2;
//         blocks += 1;
//         if alloc > (2 << memory_bits) {
//             // finished at 1073741824 27/32
//             println!("finished at {} {}/{}", alloc, blocks, 2 << block_bits);
//             break;
//         }
//     }
// }
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct UniqueStr(u32);

impl UniqueStr {
    pub fn as_str(&self) -> &str {
        let block_index = self.0 >> 28;
        let block_offset = self.0 & 0b00001111111111111111111111111111;
        // we must have initialised the interner to get an interned string
        unsafe {
            let block = &GLOBAL_INTERNER.as_ref().unwrap_unchecked().blocks[block_index as usize];
            let end = std::ptr::read_unaligned::<u32>(
                block.as_ptr().add(block_offset as usize) as *const u32
            );
            let a = (block_offset + 4) as usize;
            let b = end as usize;
            let c = block.len();
            let str = match block.get(a..b) {
                Some(s) => s,
                None => todo!(),
            };
            std::str::from_utf8_unchecked(str)
        }
    }
}

impl Debug for UniqueStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl Display for UniqueStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

#[feature(quote)]
impl quote::ToTokens for UniqueStr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let ident = Ident::new(self.as_str(), Span::call_site());
        tokens.extend(quote! {
            #ident
        });
    }
}

pub struct StringInterner<'a> {
    map: HashMap<&'a str, UniqueStr>,
    blocks: Vec<Box<[u8]>>,
    head_len: usize,
    spooky: PhantomData<&'a ()>,
}

impl StringInterner<'static> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            blocks: vec![vec![0; 4096].into_boxed_slice()],
            head_len: 0,
            spooky: PhantomData,
        }
    }
    pub fn intern(&'static mut self, string: &str) -> UniqueStr {
        if let Some(&found) = self.map.get(string) {
            return found;
        }

        let head = self.blocks.last().unwrap();
        let cap = head.len();
        if cap < (self.head_len + string.len() + 4) {
            let new_cap = ((cap + 1).max(string.len() + 4)).next_power_of_two();
            let new_head = vec![0; new_cap].into_boxed_slice();

            self.blocks.push(new_head);
            self.head_len = 0;
        }

        let block_index = self.blocks.len() - 1;
        // make borrowchecker happy
        let mut head = self.blocks.last_mut().unwrap();
        let block_offset = self.head_len;
        let string_start = block_offset + 4;
        let end = string_start + string.len();

        self.head_len = end;

        assert!(block_index <= 2 << 4);
        assert!(block_offset <= 2 << 28);

        // block_offset points at a number of the end of the string
        unsafe {
            (head.as_mut_ptr().add(block_offset) as *mut u32).write_unaligned(end as u32);
        }

        // copy string to buffer
        head[string_start..(string_start + string.len())].copy_from_slice(string.as_bytes());

        let raw = (block_offset as u32) | ((block_index as u32) << 28);
        let interned = UniqueStr(raw);

        let str = unsafe { std::str::from_utf8_unchecked(&head[string_start..(string_start + string.len())]) };
        self.map.insert(str, interned);

        interned
    }
}

static mut GLOBAL_INTERNER: Option<StringInterner> = None;

pub fn intern(string: &str) -> UniqueStr {
    unsafe {
        if GLOBAL_INTERNER.is_none() {
            GLOBAL_INTERNER = Some(StringInterner::new());
        }

        GLOBAL_INTERNER.as_mut().unwrap().intern(string)
    }
}

#[test]
fn test_interner() {
    let a1 = intern("test");
    let b1 = intern("test2");
    let a2 = intern("test");
    let b2 = intern("test2");
    assert!(a1 != b1);
    assert_eq!(a1, a2);
    assert_eq!(a1.as_str(), a2.as_str());
    assert_eq!(b1, b2);
    assert_eq!(b1.as_str(), b2.as_str());
    
    let mut set = HashSet::new();
    set.insert(a1);
    set.insert(b1);
    
    assert!(set.get(&a2).is_some());
    assert!(set.get(&b2).is_some());
}