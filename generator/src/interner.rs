use std::{collections::HashSet, hash::Hash, mem};

// somewhat naively interns strings by copying them to chunks of memory which are held alive
// for the whole lifetime of the program, effectively leaking them
#[derive(Clone, Copy, Debug)]
pub struct UniqueStr(&'static str);

impl UniqueStr {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq for UniqueStr {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}

impl Eq for UniqueStr {}

impl Hash for UniqueStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0.as_ptr(), state);
    }
}

pub struct StringInterner<'a> {
    map: HashSet<&'a str>,
    head: String,
    full: Vec<String>,
}

impl StringInterner<'static> {
    pub fn new() -> Self {
        Self {
            map: HashSet::new(),
            head: String::with_capacity(1024),
            full: Vec::new(),
        }
    }
    pub fn intern(&'static mut self, string: &str) -> UniqueStr {
        if let Some(&found) = self.map.get(string) {
            return UniqueStr(found);
        }

        let cap = self.head.capacity();
        if cap < self.head.len() + string.len() {
            let new_cap = ((cap + 1).max(string.len())).next_power_of_two();
            let new_head = String::with_capacity(new_cap);

            let old_head = mem::replace(&mut self.head, new_head);
            self.full.push(old_head);
        }

        let interned = {
            let start = self.head.len();
            // copy string to buffer
            self.head.push_str(string);
            // slice the part added to the buffer, this is our string
            let str = &self.head[start..];

            str
        };

        self.map.insert(interned);

        UniqueStr(interned)
    }
}

pub fn intern(string: &str) -> UniqueStr {
    static mut INNER_INTERNER: Option<StringInterner> = None;
    unsafe {
        if INNER_INTERNER.is_none() {
            INNER_INTERNER = Some(StringInterner::new());
        }

        INNER_INTERNER.as_mut().unwrap().intern(string)
    }
}
