use std::{
    fmt::{Display, Write},
    ops::Deref,
};

use lasso::{Rodeo, Spur};

use crate::Registry;

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
    // fucking https://docs.microsoft.com/en-us/cpp/c-language/c-bit-fields?view=msvc-170
    // of course vulkan would use that
    pub bitfield_len: Option<u32>,
}

impl TypeDecl {
    // helper function to convert immutable decls
    pub fn as_rust_order(&self, reg: &Registry) -> TypeDecl {
        let mut decl = self.clone();
        decl.make_rust_order(reg);
        decl
    }
    // reorder and modify to match rust syntax inplace
    pub fn make_rust_order(&mut self, reg: &Registry) {
        // given that the input is C, non primitive types are prefixed with "struct"
        // rust doesn't care, we need to do this here rather in the main loop as the const
        // swapping part would need to check for this and such
        self.tokens.retain(|t| match t {
            TypeToken::Ident(ident) => reg.resolve(ident).deref() != "struct",
            _ => true,
        });

        let mut i = 0;
        // Const 'char'** => 'char' Const* Mut*
        while i < self.tokens.len() {
            match self.tokens[i] {
                TypeToken::Ident(ident) => {}
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
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, reg: &Registry) -> std::fmt::Result {
        // rust cannot represent bitfields; they need to be resolved higher up, currently we store in this
        // field the total amount of bits used after merging the bitfields together, so for now this check is disabled
        // assert!(self.bitfield_len.is_none());
        for (i, token) in self.tokens.iter().enumerate() {
            let temp;
            let str = match token {
                TypeToken::Const => "const",
                TypeToken::Mut => "mut",
                TypeToken::Ptr => "*",
                TypeToken::Ident(ty) => {
                    temp = Some(reg.resolve(ty));
                    temp.as_deref().unwrap()
                }
            };
            f.write_str(str)?;
            if i != self.tokens.len() - 1 && *token != TypeToken::Ptr {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

pub fn parse_type(str: &str, has_name: bool, reg: &Registry) -> (Option<Spur>, TypeDecl) {
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
                    "const" => out.tokens.push(TypeToken::Const),
                    _ => out.tokens.push(TypeToken::Ident(reg.get_or_intern(ident))),
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
            Some(TypeToken::Ident(str)) => reg.resolve(&str).parse::<u32>().unwrap(),
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
    let reg = Registry::new();

    let expected = (
        Some(reg.get_or_intern("deviceName")),
        TypeDecl {
            tokens: vec![TypeToken::Ident(reg.get_or_intern("char")), TypeToken::Ptr],
            array_len: Some(reg.get_or_intern("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")),
            bitfield_len: None,
        },
    );
    let test = parse_type(
        "char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]",
        true,
        &reg,
    );
    assert_eq!(expected, test);

    let expected = (
        Some(reg.get_or_intern("pTest")),
        TypeDecl {
            tokens: vec![
                TypeToken::Const,
                TypeToken::Ident(reg.get_or_intern("char")),
                TypeToken::Ptr,
                TypeToken::Const,
            ],
            array_len: None,
            bitfield_len: None,
        },
    );
    let test = parse_type("const char* const pTest", true, &reg);
    assert_eq!(expected, test);
}

#[test]
fn test_type_parse_convert() {
    let reg = Registry::new();

    let c = vec![
        TypeToken::Const,
        TypeToken::Ident(reg.get_or_intern("VkAccelerationStructureBuildRangeInfoKHR")),
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
        TypeToken::Ident(reg.get_or_intern("VkAccelerationStructureBuildRangeInfoKHR")),
    ];

    let c_src = "const VkAccelerationStructureBuildRangeInfoKHR *const**";
    let (_, mut decl) = parse_type(c_src, false, &reg);

    assert_eq!(decl.tokens, c);

    decl.make_rust_order(&reg);
    assert_eq!(decl.tokens, r);
}
