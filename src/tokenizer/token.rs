use std::fmt;

#[derive(Debug, Clone)]
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
    Pipe,
    Empty
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
            Token::Pipe => String::from("|"),
            Token::Empty => String::from("None")
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
            Token::Pipe => String::from("|"),
            Token::Empty => String::from("None")
        };
        write!(f, "Unexpected token {}", tok)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        use Token::*;
        match (self, other) {
            (Keyword(_), Keyword(_)) => true,
            (NumberLiteral(_), NumberLiteral(_)) => true,
            (StringLiteral(_), StringLiteral(_)) => true,
            (BoolLiteral(_), BoolLiteral(_)) => true,
            (Identifier(_), Identifier(_)) => true,
            (BinaryOp(_), BinaryOp(_)) => true,
            (LogicalOp(_), LogicalOp(_)) => true,
            (AssignmentOp(_), AssignmentOp(_)) => true,
            (Let, Let) => true,
            (OpenParen, OpenParen) => true,
            (CloseParen, CloseParen) => true,
            (OpenBrace, OpenBrace) => true,
            (CloseBrace, CloseBrace) => true,
            (Pipe, Pipe) => true,
            (Empty, Empty) => true,
            _ => false,
        }
    }
}