use regex::{ Regex, RegexSet };
use lazy_static::*;
use std::panic;
use substring::Substring;

pub mod token;
use token::Token;

pub struct TokenValue {
    number: f64,
    string: String,
    boolean: bool
}


#[derive(Debug)]
pub struct Tokenizer {
    program: String,
    tokens: Vec<Token>,
    cursor: usize,
    len: usize
}

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            program: String::from(""),
            tokens: Vec::new(),
            cursor: 0,
            len: 0
        }
    }

    pub fn from(input: &str) -> Self {
        Tokenizer {
            program: String::from(input),
            tokens: Vec::new(),
            cursor: 0,
            len: input.len()
        }
    }

    pub fn init(&mut self, input: &str) {
        self.program = String::from(input);
        self.len = input.len();
    }

    pub fn has_more_tokens(&self) -> bool {
        self.cursor < self.len
    }

    pub fn get_next_token(&mut self, consume: bool) -> Token {
        lazy_static! {
            static ref IDENT_PATTERN: Regex = Regex::new(r#"[^\s"(){}]+"#).unwrap();
            static ref NUMBER_PATTERN: Regex = Regex::new(r"^\d+\.?\d*").unwrap();
            static ref STRING_PATTERN: Regex = Regex::new(r#"^".*""#).unwrap();
            static ref WHITESPACE_PATTERN: Regex = Regex::new(r"^[\s]+").unwrap();
            static ref LINE_TERM_PATTERN: Regex = Regex::new(r"^\n").unwrap();
            static ref LOGICAL_OP_SET: RegexSet = RegexSet::new(&[
                r"^\|\|",
                r"^\|",
                r"^&&",
                r"^&",
            ]).unwrap();
            static ref BINARY_OP_SET: RegexSet = RegexSet::new(&[
                r"^\+",
                r"^-",
                r"^\*",
                r"^/"
            ]).unwrap();
            static ref ASSIGNMENT_OP_SET: RegexSet = RegexSet::new(&[
                r"^\+=",
                r"^-=",
                r"^\*=",
                r"^/=",
                r"^="
            ]).unwrap();
        }
        let mut consumed_len = 0;
        let result: Token;
        let tok_len: usize;
        let temp_program: &str = self.program.substring(self.cursor, self.program.len());
        match WHITESPACE_PATTERN.captures_iter(temp_program).next() {
            Some(cap) => {
                self.cursor += cap.len();
                if consume {
                    consumed_len += cap.len();
                }
                return self.get_next_token(consume);
            },
            None => {}
        }
        println!("{}", temp_program);
        match temp_program {
            _ if temp_program.starts_with("#") => {
                let comment_chars: std::str::Chars = temp_program.chars();
                for (index, chr) in comment_chars.enumerate() {
                    if let '\n' = chr {
                        self.cursor += index;
                        if consume {
                            consumed_len += index;
                        }
                        break;
                    }
                }
                return self.get_next_token(consume);
            },
            _ if temp_program.starts_with("let")
                => (tok_len, result) = (3, Token::Let),
            _ if temp_program.starts_with("(")
                => (tok_len, result) = (1, Token::OpenParen),
            _ if temp_program.starts_with(")")
                => (tok_len, result) = (1, Token::CloseParen),
            _ if temp_program.starts_with("{")
                => (tok_len, result) = (1, Token::OpenBrace),
            _ if temp_program.starts_with("}")
                => (tok_len, result) = (1, Token::CloseBrace),
            _ if temp_program.starts_with("true")
                => (tok_len, result) = (4, Token::BoolLiteral(true)),
            _ if temp_program.starts_with("false")
                => (tok_len, result) = (5, Token::BoolLiteral(false)),
            _ if temp_program.starts_with("if")
                => (tok_len, result) = (2, Token::Keyword(String::from("if"))),
            _ if temp_program.starts_with("else")
                => (tok_len, result) = (4, Token::Keyword(String::from("else"))),
            _ if temp_program.starts_with("fn")
                => (tok_len, result) = (2, Token::Keyword(String::from("fn"))),
            _ if temp_program.starts_with("return")
                => (tok_len, result) = (6, Token::Keyword(String::from("return"))),
            number if NUMBER_PATTERN.is_match(&temp_program) => {
                let num_str = NUMBER_PATTERN.captures_iter(number).next().unwrap().get(0).unwrap().as_str();
                let n = match num_str.parse::<f64>() {
                    Ok(num) => num,
                    Err(e) => {println!("{}", e); panic!()}
                };
                (tok_len, result) = (num_str.len(), Token::NumberLiteral(n));
            },
            string if STRING_PATTERN.is_match(&temp_program) => {
                let string = STRING_PATTERN.captures_iter(string).next().unwrap().get(0).unwrap().as_str();
                (tok_len, result) = (string.len(), Token::StringLiteral(String::from(string))); //.substring(1, string.len()-1)
            },
            op if ASSIGNMENT_OP_SET.is_match(&temp_program) => {
                let op: &str = Regex::new(
                    &ASSIGNMENT_OP_SET
                    .patterns()[
                        ASSIGNMENT_OP_SET
                        .matches(temp_program)
                        .into_iter()
                        .next().unwrap()
                    ]
                ).unwrap()
                    .captures_iter(op)
                    .next().unwrap()
                    .get(0).unwrap()
                    .as_str();
                (tok_len, result) = (op.len(), Token::AssignmentOp(String::from(op)));
            },
            op if LOGICAL_OP_SET.is_match(&temp_program) => {
                let op: &str = Regex::new(
                    &LOGICAL_OP_SET
                    .patterns()[
                        LOGICAL_OP_SET
                        .matches(temp_program)
                        .into_iter()
                        .next().unwrap()
                    ]
                ).unwrap()
                    .captures_iter(op)
                    .next().unwrap()
                    .get(0).unwrap()
                    .as_str();
                tok_len = op.len();
                result = match op {
                    "|" => Token::Pipe,
                    other => Token::LogicalOp(String::from(other))
                };
            },
            op if BINARY_OP_SET.is_match(&temp_program) => {
                let op: &str = Regex::new(
                    &BINARY_OP_SET
                    .patterns()[
                        BINARY_OP_SET
                        .matches(temp_program)
                        .into_iter()
                        .next().unwrap()
                    ]
                ).unwrap()
                    .captures_iter(op)
                    .next().unwrap()
                    .get(0).unwrap()
                    .as_str();
                (tok_len, result) = (op.len(), Token::BinaryOp(String::from(op)));
            },
            ident if IDENT_PATTERN.is_match(&temp_program) => {
                let ident = IDENT_PATTERN.captures_iter(ident).next().unwrap().get(0).unwrap().as_str();
                (tok_len, result) = (ident.len(), Token::Identifier(String::from(ident)));
            },
            "" => return Token::Empty,
            bad_tok => panic!("Unknown token {}...", bad_tok)
        }

        self.tokens.push(result.clone());

        self.cursor += tok_len;
        self.cursor -= consumed_len;

        result
    }

    pub fn exec(&mut self) -> &Self {
        while self.has_more_tokens() {
            self.get_next_token(true);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};

    #[test]
    fn tokenizer_test_1() {
        let mut t = Tokenizer::from("if 1.0 25.0 else 3.0");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(25.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("else")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(3.0))
    }

    #[test]
    fn tokenizer_test_2() {
        let mut t = Tokenizer::from("#test\n
        1.0 && 2.0");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::LogicalOp(String::from("&&")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(2.0));
    }

    #[test]
    fn tokenizer_test_3() {
        let mut t = Tokenizer::from("if true { 1 } else { 2 }");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::BoolLiteral(true));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::OpenBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::CloseBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("else")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::OpenBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::CloseBrace);
    }

    #[test]
    fn tokenizer_test_4() {
        let mut t = Tokenizer::from("if \"test\" { 1 }");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::OpenBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::CloseBrace);
    }

    #[test]
    fn tokenizer_test_5() {
        let mut t = Tokenizer::from("if \"test\" { ls -a }");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::OpenBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Identifier(String::from("ls")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::BinaryOp(String::from("-")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Identifier(String::from("a")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::CloseBrace);
    }


    #[test]
    fn tokenizer_test_6() {
        let mut t = Tokenizer::from("if \"test\" { 2 + 2 }");

        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::OpenBrace);
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::BinaryOp(String::from("+")));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token(true);
        assert_eq!(tok, Token::CloseBrace);
    }
}