use regex::{ Regex, RegexSet };
use lazy_static::*;
use std::panic;
use substring::Substring;
use std::fmt;

#[derive(Debug)]
pub struct TokenValue {
    number: f64,
    string: String,
    boolean: bool
}

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    NumberLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Identifier(String),
    BinaryOp(String),
    LogicalOp(String),
    AssignmentOp(String),
    Let,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    None
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Keyword(s) => String::from(format!("Keyword ({})", s)),
            Token::NumberLiteral(f) => String::from(format!("NumberLiteral ({})", f)),
            Token::StringLiteral(s) => String::from(format!("StringLiteral ({})", s)),
            Token::BoolLiteral(b) => String::from(format!("BoolLiteral ({})", b)),
            Token::Identifier(s) => String::from(format!("Identifier ({})", s)),
            Token::BinaryOp(s) => String::from(format!("BinaryOp ({})", s)),
            Token::LogicalOp(s) => String::from(format!("LogicalOp ({})", s)),
            Token::AssignmentOp(s) => String::from(format!("AssignmentOp ({})", s)),
            Token::Let => String::from("Let"),
            Token::OpenParen => String::from("("),
            Token::CloseParen => String::from(")"),
            Token::OpenBrace => String::from("{"),
            Token::CloseBrace => String::from("}"),
            Token::None => String::from("None")
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tok = match self {
            Token::Keyword(s) => String::from(format!("{}", s)),
            Token::NumberLiteral(f) => String::from(format!("NumberLiteral ({})", f)),
            Token::StringLiteral(s) => String::from(format!("{}", s)),
            Token::BoolLiteral(b) => String::from(format!("{}", b)),
            Token::Identifier(s) => String::from(format!("{}", s)),
            Token::BinaryOp(s) => String::from(format!("{}", s)),
            Token::LogicalOp(s) => String::from(format!("{}", s)),
            Token::AssignmentOp(s) => String::from(format!("{}", s)),
            Token::Let => String::from("Let"),
            Token::OpenParen => String::from("("),
            Token::CloseParen => String::from(")"),
            Token::OpenBrace => String::from("{"),
            Token::CloseBrace => String::from("}"),
            Token::None => String::from("None")
        };
        write!(f, "Unexpected token {}", tok)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        use Token::*;
        match (self, other) {
            (Keyword(a), Keyword(b)) => true,
            (NumberLiteral(a), NumberLiteral(b)) => true,
            (StringLiteral(a), StringLiteral(b)) => true,
            (BoolLiteral(a), BoolLiteral(b)) => true,
            (Identifier(a), Identifier(b)) => true,
            (BinaryOp(a), BinaryOp(b)) => true,
            (LogicalOp(a), LogicalOp(b)) => true,
            (AssignmentOp(a), AssignmentOp(b)) => true,
            (Let, Let) => true,
            (OpenParen, OpenParen) => true,
            (CloseParen, CloseParen) => true,
            (OpenBrace, OpenBrace) => true,
            (CloseBrace, CloseBrace) => true,
            (None, None) => true,
            _ => false,
        }
    }
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

    pub fn get_next_token(&mut self) -> &mut Token {
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
                r"^/="
            ]).unwrap();
        }

        let result: Token;
        let tok_len: usize;
        let temp_program: &str = self.program.substring(self.cursor, self.program.len());
        match WHITESPACE_PATTERN.captures_iter(temp_program).next() {
            Some(cap) => {
                self.cursor += cap.len();
                return self.get_next_token();
            },
            None => {}
        }
        match temp_program {
            _ if temp_program.starts_with("#") => {
                let comment_chars = temp_program.chars();
                for (index, chr) in comment_chars.enumerate() {
                    if let '\n' = chr {
                        self.cursor += index;
                        break;
                    }
                }
                return self.get_next_token();
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
                (tok_len, result) = (string.len(), Token::StringLiteral(String::from(string.substring(1, string.len()-1))));
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
                (tok_len, result) = (op.len(), Token::LogicalOp(String::from(op)));
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
                (tok_len, result) = (op.len(), Token::LogicalOp(String::from(op)));
            },
            ident if IDENT_PATTERN.is_match(&temp_program) => {
                let ident = IDENT_PATTERN.captures_iter(ident).next().unwrap().get(0).unwrap().as_str();
                (tok_len, result) = (ident.len(), Token::Identifier(String::from(ident)));
            },
            bad_tok => panic!("Unknown token {}...", bad_tok.substring(0, 7))
        }

        self.tokens.push(result);

        self.cursor += tok_len;

        self.tokens.last_mut().unwrap()
    }

    pub fn exec(&mut self) -> &Self {
        while self.has_more_tokens() {
            self.get_next_token();
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

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(25.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("else")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(3.0))
    }

    #[test]
    fn tokenizer_test_2() {
        let mut t = Tokenizer::from("#test\n
        1.0 && 2.0");

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::LogicalOp(String::from("&&")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(2.0));
    }

    #[test]
    fn tokenizer_test_3() {
        let mut t = Tokenizer::from("if true { 1 } else { 2 }");

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::BoolLiteral(true));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::OpenBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::CloseBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("else")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::OpenBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::CloseBrace);
    }

    #[test]
    fn tokenizer_test_4() {
        let mut t = Tokenizer::from("if \"test\" { 1 }");

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::OpenBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(1.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::CloseBrace);
    }

    #[test]
    fn tokenizer_test_5() {
        let mut t = Tokenizer::from("if \"test\" { ls -a }");

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::OpenBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Identifier(String::from("ls")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::BinaryOp(String::from("-")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Identifier(String::from("a")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::CloseBrace);
    }


    #[test]
    fn tokenizer_test_6() {
        let mut t = Tokenizer::from("if \"test\" { 2 + 2 }");

        let tok = t.get_next_token();
        assert_eq!(*tok, Token::Keyword(String::from("if")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::StringLiteral(String::from("test")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::OpenBrace);
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::BinaryOp(String::from("+")));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::NumberLiteral(2.0));
        let tok = t.get_next_token();
        assert_eq!(*tok, Token::CloseBrace);
    }
}