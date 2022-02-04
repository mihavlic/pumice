use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Preprocessor(&'static str),
    Newline,
    Comma,
    Ident(String),
    Num(i32),
    Operator(Op),
    LParen,
    RParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    Not,
    And,
    Or,
}

pub fn lex(string: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    let mut chars = string.chars();

    let mut next;

    let mut peek = match chars.next() {
        Some(char) => char,
        None => return tokens,
    };

    loop {
        next = peek;
        peek = chars.next().unwrap_or('\0');

        if next == '\0' {
            break;
        }

        loop {
            let op = match (next, peek) {
                ('/', '/') => {
                    // eat comment
                    while !matches!(chars.next(), None | Some('\n')) {}

                    next = '\n';
                    peek = chars.next().unwrap_or('\0');

                    continue;
                }
                ('=', '=') => Op::Eq,
                ('!', '=') => Op::Neq,
                ('<', '=') => Op::Lte,
                ('>', '=') => Op::Gte,
                _ => break,
            };

            tokens.push(Token::Operator(op));

            chars.next();
            peek = chars.next().unwrap_or('\0');

            continue;
        }

        let token = match next {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Operator(Op::Add),
            '-' => Token::Operator(Op::Sub),
            '*' => Token::Operator(Op::Mul),
            '%' => Token::Operator(Op::Mod),
            '/' => Token::Operator(Op::Div),
            '>' => Token::Operator(Op::Gt),
            '<' => Token::Operator(Op::Lt),
            '#' => {
                buf.push(peek);
                loop {
                    match chars.next() {
                        None | Some(' ' | '\n') => {
                            const DIRECTIVES: &[&str] =
                                &["if", "ifdef", "ifndev", "elif", "else", "endif", "define"];
                            let directive = DIRECTIVES
                                .iter()
                                .find(|dir| **dir == buf.as_str())
                                .expect(&format!("Unknown preprocessor directive {}", buf));
                            buf.clear();

                            peek = chars.next().unwrap_or('\0');
                            break Token::Preprocessor(directive);
                        }
                        Some(other) => buf.push(other),
                    }
                }
            }
            '\\' => match peek {
                '\n' => {
                    chars.next();
                    peek = chars.next().unwrap_or('\0');
                    continue;
                }
                _ => unreachable!("Unsupported escaped character"),
            },
            ',' => Token::Comma,
            '\n' => Token::Newline,
            ' ' => continue,
            _ => {
                buf.push(next);

                if !peek.is_ascii_alphanumeric() && peek != '_' {
                    let token = match buf.parse::<i32>() {
                        Ok(num) => Token::Num(num),
                        Err(_) => Token::Ident(buf.clone()),
                    };

                    tokens.push(token);
                    buf.clear();
                }
                continue;
            }
        };

        tokens.push(token);
    }

    tokens
}

pub struct PreprocessorContext {
    defines: HashMap<String, i32>,
}

impl PreprocessorContext {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
        }
    }
    pub fn process(&mut self, tokens: &mut Vec<Token>) {
        let mut lexer = tokens.iter_mut().peekable();

        loop {
            let next = match lexer.next() {
                Some(token) => token,
                None => break,
            };

            match next {
                Token::Ident(ident) => {
                    let list = parse_paren_list(&mut lexer);

                    let value;

                    if ident == "defined" {
                        let list = list.unwrap();
                        assert!(list.len() == 1);
                        value = self.defines.contains_key(&list[0]) as i32;
                    } else if let Some(val) = self.defines.get(ident) {
                        value = *val;
                    } else {
                        continue;
                    }

                    *next = Token::Num(value);
                }
                _ => continue,
            }
        }

        let mut processed = Vec::new();

        let mut lexer = tokens.iter().peekable();
        let mut prev_branch = None;
        loop {
            let next = match lexer.next() {
                Some(token) => token,
                None => break,
            };

            match next {
                Token::Preprocessor(dir) => match *dir {
                    "if" => {
                        let val =parse_expr(&mut lexer, 0).is_positive();
                        if val {
                            pass_preprocessor_block(&mut lexer, &mut processed);
                        } else {
                            discard_preprocessor_block(&mut lexer);
                        }
                        prev_branch = Some(val);
                    }
                    "ifdef" => todo!(),
                    "ifndev" => todo!(),
                    "elif" => {
                        if prev_branch == Some(true) {
                            discard_preprocessor_block(&mut lexer);
                            continue;
                        }
                        let val = parse_expr(&mut lexer, 0).is_positive();
                        if val {
                            pass_preprocessor_block(&mut lexer, &mut processed);
                        }
                        prev_branch = Some(val);
                    }
                    "else" => {
                        if prev_branch == Some(false) {
                            pass_preprocessor_block(&mut lexer, &mut processed);
                        } else {
                            discard_preprocessor_block(&mut lexer);
                        }
                    }
                    "endif" => {
                        prev_branch = None;
                    }
                    "define" => todo!(),
                    _ => unreachable!(),
                },
                _ => processed.push(next.clone()),
            }
        }

        fn pass_preprocessor_block(lexer: &mut Lexer, output: &mut Vec<Token>) {
            loop {
                match lexer.peek() {
                    None | Some(Token::Preprocessor(_)) => break,
                    Some(&other) => output.push(other.clone()),
                }
                lexer.next();
            }
        }
        fn discard_preprocessor_block(lexer: &mut Lexer) {
            loop {
                match lexer.peek() {
                    None | Some(Token::Preprocessor(_)) => break,
                    Some(_) => (),
                }
                lexer.next();
            }
        }

        *tokens = processed;
    }
}

fn op_prec(op: Op) -> i32 {
    match op {
        Op::Or => 1,
        Op::And => 2,
        Op::Lt | Op::Gt | Op::Lte | Op::Gte => 3,
        Op::Eq | Op::Neq => 4,
        Op::Add | Op::Sub => 5,
        Op::Mul | Op::Div | Op::Mod => 6,
        _ => -1,
    }
}

fn parse_paren_list(
    lexer: &mut std::iter::Peekable<std::slice::IterMut<Token>>,
) -> Option<Vec<String>> {
    if matches!(lexer.peek(), Some(Token::LParen)) {
        lexer.next();
        let mut vec = Vec::new();

        loop {
            match lexer.next() {
                Some(Token::Ident(ident)) => vec.push(ident.clone()),
                None | Some(Token::RParen) => break,
                _ => unreachable!(),
            }

            assert!(matches!(lexer.next(), Some(Token::Comma)));
        }

        Some(vec)
    } else {
        None
    }
}

fn mono_op_apply(op: Op, a: i32) -> i32 {
    match op {
        Op::Not => !a.is_positive() as i32,
        Op::Sub => -a,
        _ => unreachable!(),
    }
}

fn op_apply(op: Op, a: i32, b: i32) -> i32 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => a / b,
        Op::Mod => a % b,
        Op::Eq => ((a.is_positive()) == (b.is_positive())) as i32,
        Op::Neq => ((a.is_positive()) != (b.is_positive())) as i32,
        Op::Lt => ((a.is_positive()) < (b.is_positive())) as i32,
        Op::Gt => ((a.is_positive()) > (b.is_positive())) as i32,
        Op::Lte => ((a.is_positive()) <= (b.is_positive())) as i32,
        Op::Gte => ((a.is_positive()) >= (b.is_positive())) as i32,
        Op::And => ((a.is_positive()) && (b.is_positive())) as i32,
        Op::Or => ((a.is_positive()) || (b.is_positive())) as i32,
        // special case
        // Op::Not => !b.is_positive() as i32,
        _ => unreachable!(),
    }
}

type Lexer<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

fn parse_monop(lexer: &mut Lexer<'_>) -> i32 {
    match lexer.next() {
        Some(Token::Operator(op @ (Op::Sub | Op::Not))) => {
            let arg = parse_monop(lexer);
            return mono_op_apply(*op, arg);
        }
        Some(Token::LParen) => {
            return parse_expr(lexer, 0);
        }
        Some(Token::Num(num)) => {
            // lexer.next();
            return *num;
        }
        oo => panic!("Unsupported token '{:?}'", oo),
    }
}

fn parse_expr(lexer: &mut Lexer, prec: i32) -> i32 {
    let mut lhs = parse_monop(lexer);

    loop {
        match lexer.peek() {
            Some(Token::RParen) => {
                lexer.next();
                break;
            }
            Some(Token::Operator(op)) => {
                let precedence = op_prec(*op);

                if prec <= precedence {
                    lexer.next();
                    let rhs = parse_expr(lexer, precedence + 1);
                    lhs = op_apply(*op, lhs, rhs);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    return lhs;
}

// fn parse_binop(lexer: &mut Lexer, prec: i32) -> Result<Node, ParseError> {
//     let mut lhs = parse_monop(lexer)?;

//     loop {
//         match lexer.peek() {
//             Token::BinOp(op) if prec <= op_prec(op) => {
//                 lexer.next();
//                 let rhs = parse_binop(lexer, op_prec(op) + 1)?;
//                 lhs = Node::BinOp(op, Box::new(lhs), Box::new(rhs));
//             }
//             _ => break Ok(lhs),
//         }
//     }
// }

// fn eval(tokens: &[Token], mut index: usize, prev: i32, prev_precedence: u32) -> i32 {
//     let op = tokens.get(*index);

//     let cur = match op {
//         // Some(Token::Common('('))
//         None => return 0,
//         Some(a) => a
//     };

//     index += 1;
//     while let Some(r) = tokens.get(index) {
//         match *r {
//             Token::Common('(')
//         }
//     }

//     let prec = precedence(cur);

//     if prev_precedence < prec {
//         match cur {
//             Token::BinOp('*') => 6,
//             Token::BinOp('%') => 6,
//             Token::BinOp('+') => 5,
//             Token::BinOp('-') => 5,
//             Token::BinOp('<') => 3,
//             Token::BinOp('>') => 3,
//             Token::LowerThan => 3,
//             Token::GreaterThan => 3,
//             Token::Equals => 2,
//             _ => unreachable!()
//         }
//     } else {
//         return 0;
//     }
// }

// fn parse_binop(tokens: &Vec<token>, lexer: &mut usize, prec: i32) -> (Token, i32) {
//     let mut lhs = tokens[*lexer];

//     loop {
//         match tokens[*lexer + 1] {
//             Token::Operator(op) if prec <= precedence(op) => {
//                 lexer.next();
//                 let rhs = parse_binop(lexer, op_prec(op) + 1)?;
//                 lhs = Node::BinOp(op, Box::new(lhs), Box::new(rhs));
//             }
//             _ => break Ok(lhs),
//         }
//     }
// }

// 1 - 2 * 3
