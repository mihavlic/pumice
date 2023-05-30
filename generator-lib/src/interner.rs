use std::{
    alloc::Layout,
    cell::{Ref, RefCell, RefMut, UnsafeCell},
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    mem::{align_of, size_of},
    ptr::NonNull,
};

/// Yikes this is good code

struct StringHeader {
    current: &'static str,
    original: &'static str,
}

#[derive(Clone, Copy)]
pub struct UniqueStr {
    ptr: NonNull<StringHeader>,
    #[cfg(debug_assertions)]
    guard: *const bool,
}

impl UniqueStr {
    pub fn resolve(&self) -> &str {
        self.get_header().current
    }
    pub fn resolve_original(&self) -> &str {
        self.get_header().original
    }
    pub fn eq_resolve(&self, other: UniqueStr) -> bool {
        let s = self.get_header();
        let other = other.get_header();

        s.current == other.current
    }
    pub fn is_original(&self) -> bool {
        let s = self.get_header();
        s.current == s.original
    }
    pub fn rename(&self, to: UniqueStr) {
        let s = self.get_header_mut();
        let to = to.get_header();

        // do we want renames to be infectious in this way?
        s.current = to.current;
    }
    pub fn rename_trim_prefix(&self, bytes_len: usize) {
        let s = self.get_header_mut();
        s.current = &s.current[bytes_len..];
    }
    fn get_header(&self) -> &StringHeader {
        #[cfg(debug_assertions)]
        self.check_guard();

        unsafe { self.ptr.as_ref() }
    }
    fn get_header_mut(&self) -> &mut StringHeader {
        #[cfg(debug_assertions)]
        self.check_guard();
        // ??
        let mut copy = self.ptr;

        unsafe { copy.as_mut() }
    }
    #[cfg(debug_assertions)]
    fn check_guard(&self) {
        assert!(
            unsafe { *self.guard == true },
            "Use of UniqueStr after its Interner has been dropped!"
        );
    }
}

impl PartialEq for UniqueStr {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for UniqueStr {}

impl PartialOrd for UniqueStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ptr.partial_cmp(&other.ptr)
    }
}

impl Ord for UniqueStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ptr.cmp(&other.ptr)
    }
}

impl Hash for UniqueStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl Debug for UniqueStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\"{}\"", self.resolve()))
    }
}

impl Display for UniqueStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.resolve())
    }
}

struct BumpAllocator {
    blocks: Vec<(*mut u8, usize)>,
    head_offset: usize,
}

const INITIAL_BLOCK_SIZE: usize = 4096;

impl BumpAllocator {
    fn new() -> Self {
        unsafe {
            let alloc = std::alloc::alloc(Layout::from_size_align(INITIAL_BLOCK_SIZE, 1).unwrap());
            Self {
                blocks: vec![(alloc, INITIAL_BLOCK_SIZE)],
                head_offset: 0,
            }
        }
    }
    unsafe fn alloc_block_for<T>(&mut self) -> *mut T {
        self.alloc_block(size_of::<T>(), align_of::<T>()) as *mut _
    }
    unsafe fn alloc_block(&mut self, size: usize, align: usize) -> *mut u8 {
        assert!(align != 0);

        loop {
            let &(start, cap) = self.blocks.last().unwrap();

            // Here we may construct an out-of-bounds pointer and miri complains that this is undefined behaviour:
            //   "Undefined Behavior: pointer arithmetic failed: alloc63097 has size 4096, so pointer to 4096 bytes starting at offset 67 is out-of-bounds"
            // however here:
            //   https://doc.rust-lang.org/beta/reference/behavior-considered-undefined.html
            //   https://stackoverflow.com/a/59554415
            // It is said that unless it's dereferenced it's not a problem, given that this pattern of bounds checking through pointers is quite common in C
            // I don't see how this can be undefined behaviour?
            //
            // Okay so the problem is that this function has weird rules
            //   https://doc.rust-lang.org/std/primitive.pointer.html#method.add
            //   "Both the starting and resulting pointer must be either in bounds or one byte past the end of the same allocated object."
            // So it is fine to dangle, it just can't dangle too much, thanks?
            // This is left for posterity, it's fixed now.

            let start_offset = self.head_offset + start.add(self.head_offset).align_offset(align);
            let end_offset = start_offset + size;

            if end_offset < cap {
                self.head_offset = end_offset;
                return start.add(start_offset);
            } else {
                let new_cap = (cap + 1).max(size + align - 1).next_power_of_two();
                let alloc = std::alloc::alloc(Layout::from_size_align(new_cap, 1).unwrap());

                self.blocks.push((alloc, new_cap));
                self.head_offset = 0;

                // we now have enough capacity, repeat the calculation above and return a pointer
                continue;
            }
        }
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        for &(start, cap) in &self.blocks {
            unsafe {
                std::alloc::dealloc(start, Layout::from_size_align(cap, 1).unwrap());
            }
        }
    }
}

////

/// !!! This will leak a byte of memory for the guard on every call to ::new() so that any UniqueStr's can check against it.
/// If an Rc was used it wouldn't be sensible to have the tokens be Copy and I'm not doing that.
pub struct StringInterner {
    // not really static but contained in an allocation owned by this struct, very cursed
    map: HashMap<&'static str, NonNull<StringHeader>>,
    // unsafecell because we are creating pointers from which the allocator's memory may be modified
    allocator: UnsafeCell<BumpAllocator>,
    #[cfg(debug_assertions)]
    guard: *mut bool,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            allocator: UnsafeCell::new(BumpAllocator::new()),
            #[cfg(debug_assertions)]
            guard: vec![true].leak().as_mut_ptr(),
        }
    }
    pub fn intern(&mut self, str: &str) -> UniqueStr {
        let len = str.bytes().len();
        assert!(len < u32::MAX as usize);

        if let Some(&ptr) = self.map.get(str) {
            return UniqueStr {
                ptr,
                #[cfg(debug_assertions)]
                guard: self.guard,
            };
        }

        unsafe {
            let allocator = self.allocator.get_mut();

            let text = allocator.alloc_block(len, 1);
            std::ptr::copy_nonoverlapping(str.as_ptr(), text, len);

            let header = allocator.alloc_block_for::<StringHeader>();
            // now we've lost the lifetime of the self reference! job well done
            let str = std::str::from_utf8_unchecked(std::slice::from_raw_parts(text, len));

            header.write(StringHeader {
                current: str,
                original: str,
            });

            let nonnull = NonNull::new(header).unwrap();
            self.map.insert(str, nonnull);

            UniqueStr {
                ptr: nonnull,
                #[cfg(debug_assertions)]
                guard: self.guard,
            }
        }
    }
    pub fn get_all_strings(&self) -> Vec<UniqueStr> {
        self.map
            .iter()
            .map(|(_, &ptr)| UniqueStr {
                ptr,
                #[cfg(debug_assertions)]
                guard: self.guard,
            })
            .collect()
    }
}

impl Drop for StringInterner {
    fn drop(&mut self) {
        // somehow rust is smart enough to run destructors for the other fields here?
        // if I ptr::drop_in_place the fields I get cool crashes
        #[cfg(debug_assertions)]
        unsafe {
            self.guard.write(false);
        }
    }
}

pub struct Interner(RefCell<StringInterner>);

impl Interner {
    pub fn new() -> Self {
        Interner(RefCell::new(StringInterner::new()))
    }
    pub fn intern(&self, str: &str) -> UniqueStr {
        self.0.borrow_mut().intern(str)
    }
    pub fn try_intern(&self, str: &str) -> Option<UniqueStr> {
        let s = self.0.borrow_mut();
        s.map.get(str).map(|&h| UniqueStr {
            ptr: h,
            #[cfg(debug_assertions)]
            guard: s.guard,
        })
    }
    pub fn lookup_rename<'a>(&'a self, str: &'a str) -> &'a str {
        self.0
            .borrow_mut()
            .map
            .get(str)
            .map(|&h| unsafe { h.as_ref().current })
            .unwrap_or(str)
    }
    pub fn borrow(&self) -> Ref<StringInterner> {
        self.0.borrow()
    }
    pub fn borrow_mut(&self) -> RefMut<StringInterner> {
        self.0.borrow_mut()
    }
}

pub trait Intern {
    fn intern(&self, int: &Interner) -> UniqueStr;
    fn try_intern(&self, int: &Interner) -> Option<UniqueStr>;
}

impl<T: AsRef<str>> Intern for T {
    fn intern(&self, int: &Interner) -> UniqueStr {
        int.intern(self.as_ref())
    }
    fn try_intern(&self, int: &Interner) -> Option<UniqueStr> {
        int.try_intern(self.as_ref())
    }
}

#[test]
fn test_interner() {
    let int = Interner::new();

    let a = "A".intern(&int);
    let b = "B".intern(&int);
    let s = "ඞ".intern(&int);

    assert_eq!(a.resolve(), "A");
    assert_eq!(b.resolve(), "B");
    assert_eq!(s.resolve(), "ඞ");

    a.rename(b);

    assert_eq!(a.resolve(), "B");

    let buf = vec![b'A'; INITIAL_BLOCK_SIZE];
    // unsafe because for some reason even extending a exact-capacity String is incredibly slow in miri
    let large = unsafe { std::str::from_utf8_unchecked(&buf) };

    let large_i = large.intern(&int);
    assert_eq!(large_i.resolve(), large);
}
