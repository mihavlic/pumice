use crate::interner::{intern, UniqueStr};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TypeDecl {
    tokens: Vec<TypeToken>,
    array_len: Option<UniqueStr>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeToken {
    Const,
    Ptr,
    Ident(UniqueStr),
}

pub fn parse_type(str: &str) -> (UniqueStr, TypeDecl) {
    let mut out = TypeDecl {
        tokens: Vec::new(),
        array_len: None,
    };
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
                    _ => out.tokens.push(TypeToken::Ident(intern(ident))),
                }
                start = None;
            }
        }

        match char {
            '*' => out.tokens.push(TypeToken::Ptr),
            // char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]
            ']' => {
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
            _ => {}
        }
    }

    let name = match out.tokens.pop() {
        Some(TypeToken::Ident(str)) => str,
        _ => unreachable!(),
    };

    (name, out)
}

#[test]
fn test_parse_type() {
    let expected = (
        intern("deviceName"),
        TypeDecl {
            tokens: vec![TypeToken::Ident(intern("char")), TypeToken::Ptr],
            array_len: Some(intern("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")),
        },
    );
    let test = parse_type("char deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]");
    assert_eq!(expected, test);

    let expected = (
        intern("pTest"),
        TypeDecl {
            tokens: vec![
                TypeToken::Const,
                TypeToken::Ident(intern("char")),
                TypeToken::Ptr,
                TypeToken::Const,
            ],
            array_len: None,
        },
    );
    let test = parse_type("const char* const pTest");
    assert_eq!(expected, test);
}
