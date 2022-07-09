use std::{
    fmt::{Debug, Write},
    ops::Deref,
};

use lasso::Spur;

use crate::{Intern, Interner, Resolve};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeToken {
    Ident(Spur),
    Ptr,
    Const,
    // only in rust-ed type declarations
    Mut,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeDecl {
    pub tokens: Vec<TypeToken>,
    // only one of these should be possible at a time
    pub array_len: Option<Spur>,
    // bitfield! yay! https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170
    pub bitfield_len: Option<u32>,
}

impl TypeDecl {
    // helper function to convert immutable decls
    pub fn as_rust_order(&self, int: &Interner) -> TypeDecl {
        let mut decl = self.clone();
        decl.make_rust_order(int);
        decl
    }
    // reorder and modify to match rust syntax inplace
    pub fn make_rust_order(&mut self, int: &Interner) {
        // given that the input is C, non primitive types are prefixed with "struct"
        // rust doesn't care, we need to do this here rather in the main loop as the const
        // swapping part would need to check for this and such
        self.tokens.retain(|t| match t {
            TypeToken::Ident(ident) => int.resolve(ident).deref() != "struct",
            _ => true,
        });

        let mut i = 0;
        // Const 'char'** => 'char' Const* Mut*
        while i < self.tokens.len() {
            match self.tokens[i] {
                TypeToken::Ident(_ident) => {}
                // 'char'* => char Mut *
                TypeToken::Ptr => {
                    if let Some(prev) = i.checked_sub(1) {
                        match self.tokens[prev] {
                            TypeToken::Const => {}
                            _ => {
                                self.tokens.insert(i, TypeToken::Mut);
                                i += 1;
                            }
                        }
                    }
                }
                // Const 'char' => 'char' Const
                TypeToken::Const => {
                    if let Some(TypeToken::Ident(_)) = self.tokens.get(i + 1) {
                        self.tokens.swap(i, i + 1);
                        i += 1;
                    }
                }
                TypeToken::Mut => unreachable!(),
            }
            i += 1;
        }
        // 'char' Const* Mut* => *Mut *Const char
        self.tokens.reverse();
    }
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, int: &Interner) -> std::fmt::Result {
        // rust cannot represent bitfields; they need to be resolved higher up, currently we store in this
        // field the total amount of bits used after merging the bitfields together, so for now this check is disabled
        // assert!(self.bitfield_len.is_none());
        let mut tokens = self.tokens.as_slice();
        // remove the outer pointer decoration in the type as we'll be replacing it with an array
        if self.array_len.is_some() {
            tokens = &tokens[..tokens.len() - 1];
        }

        for (i, token) in tokens.iter().enumerate() {
            let temp;
            let str = match token {
                TypeToken::Const => "const",
                TypeToken::Mut => "mut",
                TypeToken::Ptr => "*",
                TypeToken::Ident(ty) => {
                    temp = Some(int.resolve(ty));
                    temp.as_deref().unwrap()
                }
            };
            f.write_str(str)?;
            if i != tokens.len() - 1 && *token != TypeToken::Ptr {
                f.write_char(' ')?;
            }
        }
        if let Some(size) = self.array_len {
            f.write_fmt(format_args!("[{}]", size.resolve(int)))?;
        }
        Ok(())
    }
}

pub fn parse_type(str: &str, has_name: bool, int: &Interner) -> (Option<Spur>, TypeDecl) {
    let mut out = TypeDecl {
        tokens: Vec::new(),
        array_len: None,
        bitfield_len: None,
    };
    let mut bitfield = false;
    let mut start = None;
    let mut chars = str.char_indices().peekable();
    loop {
        let (mut i, char) = match chars.next() {
            Some(s) => s,
            None => break,
        };

        let alphanumeric = char.is_ascii_alphanumeric() || char == '_';
        if alphanumeric {
            if start.is_none() {
                start = Some(i);
            }
        }

        if alphanumeric == false || chars.peek().is_none() {
            if let Some(s) = start {
                if alphanumeric == true && chars.peek().is_none() {
                    i += 1;
                }

                let ident = &str[s..i];
                match ident {
                    // archaic C declaration like "struct Type something" are read as "Type something"
                    "struct" => {}
                    "const" => out.tokens.push(TypeToken::Const),
                    _ => out.tokens.push(TypeToken::Ident(ident.intern(int))),
                }
                start = None;
            }
        }

        match char {
            '*' => out.tokens.push(TypeToken::Ptr),
            // char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]
            ']' => {
                assert!(bitfield == false);
                match out.tokens.pop() {
                    Some(TypeToken::Ident(str)) => {
                        // make <something>  <name>[len]
                        // into <something>* <name>
                        out.array_len = Some(str);
                        out.tokens.insert(out.tokens.len() - 1, TypeToken::Ptr)
                    }
                    _ => unreachable!(),
                }
            }
            ':' => bitfield = true,
            _ => {}
        }
    }

    if bitfield {
        assert!(out.array_len.is_none());
        let size = match out.tokens.pop() {
            Some(TypeToken::Ident(str)) => int.resolve(&str).parse::<u32>().unwrap(),
            _ => unreachable!(),
        };
        out.bitfield_len = Some(size);
    }

    let name = if has_name {
        Some(match out.tokens.pop() {
            Some(TypeToken::Ident(str)) => str,
            _ => unreachable!(),
        })
    } else {
        None
    };

    (name, out)
}

#[test]
fn test_parse_type() {
    let int = Interner::new();

    let expected = (
        Some("deviceName".intern(&int)),
        TypeDecl {
            tokens: vec![TypeToken::Ident("char".intern(&int)), TypeToken::Ptr],
            array_len: Some("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE".intern(&int)),
            bitfield_len: None,
        },
    );
    let test = parse_type(
        "char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]",
        true,
        &int,
    );
    assert_eq!(expected, test);

    let expected = (
        Some("pTest".intern(&int)),
        TypeDecl {
            tokens: vec![
                TypeToken::Const,
                TypeToken::Ident("char".intern(&int)),
                TypeToken::Ptr,
                TypeToken::Const,
            ],
            array_len: None,
            bitfield_len: None,
        },
    );
    let test = parse_type("const char* const pTest", true, &int);
    assert_eq!(expected, test);
}

#[test]
fn test_type_parse_convert() {
    let int = Interner::new();

    let c = vec![
        TypeToken::Const,
        TypeToken::Ident("VkAccelerationStructureBuildRangeInfoKHR".intern(&int)),
        TypeToken::Ptr,
        TypeToken::Const,
        TypeToken::Ptr,
        TypeToken::Ptr,
    ];
    let r = vec![
        TypeToken::Ptr,
        TypeToken::Mut,
        TypeToken::Ptr,
        TypeToken::Const,
        TypeToken::Ptr,
        TypeToken::Const,
        TypeToken::Ident("VkAccelerationStructureBuildRangeInfoKHR".intern(&int)),
    ];

    let c_src = "const VkAccelerationStructureBuildRangeInfoKHR *const**";
    let (_, mut decl) = parse_type(c_src, false, &int);

    assert_eq!(decl.tokens, c);

    decl.make_rust_order(&int);
    assert_eq!(decl.tokens, r);
}
