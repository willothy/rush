use std::boxed::Box;
use std::mem;
use std::fmt;

use crate::tokenizer::{Token, Tokenizer};

type Result<Token> = std::result::Result<Token, SyntaxError>;

pub enum SyntaxError {
    UnexpectedToken(String)
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tok = match self {
            SyntaxError::UnexpectedToken(t) => t,
        };
        write!(f, "Unexpected token {}", tok)
    }
}

pub enum ASTNodeType {
    StatementList(Vec<ASTNode>),
    Statement(Box<ASTNode>),
    Expression(Box<ASTNode>),
    VarDef(String, Box<ASTNode>),
    StringLiteral(String),
    NumberLiteral(f64),
    BoolLiteral(bool),
    Argslist,
    None
}

pub struct ASTNode {
    node_type: ASTNodeType,
}

pub struct Parser {
    lookahead: Token,
    tokenizer: Tokenizer
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            lookahead: Token::None,
            tokenizer: Tokenizer::new()
        }
    }

    pub fn parse(&mut self, input: &str) -> ASTNode {
        self.tokenizer.init("");
        self.update_lookahead();
        return ASTNode {
            node_type: ASTNodeType::StatementList(self.statement_list())
        }
    }

    fn update_lookahead(&mut self) {
        self.lookahead = mem::replace(self.tokenizer.get_next_token(), Token::None);
    }

    fn get_lookahead(&mut self) -> Token {
        mem::replace(self.tokenizer.get_next_token(), Token::None)
    }

    fn expect(&mut self, tok_type: Token) -> Result<Token> {
        let token = self.get_lookahead();

        if token == tok_type {
            self.update_lookahead();
            return Ok(token)
        }
        Err(SyntaxError::UnexpectedToken(token.to_string()))
    }

    /**
     *  StatementList
     *  : Statement
     *  | Statement LOGICAL_OP StatementList
     */
    pub fn statement_list(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::<ASTNode>::new();

        while self.lookahead != Token::None {
            statements.push(self.statement());
        }
        statements
    }

    /**
     *  Statement
     *  : IfStatement
     *  | Expression
     */
    pub fn statement(&mut self) -> ASTNode {
        let look = self.get_lookahead();
        match look {
            Token::Keyword(s) if s == String::from("if")
                => self.if_statement(),
            Token::Keyword(s) if s == String::from("fn")
                => self.fn_def(),
            Token::Identifier(s)
                => self.command(s),
            Token::Let
                => self.var_def(),
            _ => panic!(),
        }
    }

    /**
     *  Expression
     *  : BinaryExpression
     *  | Command Argslist
     */
    pub fn expression(&mut self) -> ASTNode {
        let look = self.get_lookahead();
        match look {
            Token::StringLiteral(string) => match self.expect(Token::StringLiteral(string)) {
                Ok(Token::StringLiteral(s)) => ASTNode {
                    node_type: ASTNodeType::StringLiteral(s)
                },
                Err(e) => panic!("{}", e),
                _ => panic!()
            },
            Token::NumberLiteral(number) => match self.expect(Token::NumberLiteral(number)) {
                Ok(Token::NumberLiteral(n)) => ASTNode {
                    node_type: ASTNodeType::NumberLiteral(n)
                },
                Err(e) => panic!("{}", e),
                _ => panic!()
            },
            Token::BoolLiteral(boolean) => match self.expect(Token::BoolLiteral(boolean)) {
                Ok(Token::BoolLiteral(b)) => ASTNode {
                    node_type: ASTNodeType::BoolLiteral(b)
                },
                Err(e) => panic!("{}", e),
                _ => panic!()
            },
            Token::OpenParen => match self.expect(Token::OpenParen) {
                Ok(Token::OpenParen) => self.parenthesized_expression(),
                Err(e) => panic!("{}", e),
                _ => panic!()
            }
            _ => panic!()
        }
    }

    pub fn parenthesized_expression(&mut self) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::None
        }
    }

    pub fn var_def(&mut self) -> ASTNode {
        self.expect(Token::Let);
        let name: String = match self.expect(Token::Identifier(String::new())) {
            Ok(Token::Identifier(name)) => name,
            _ => { panic!(); }
        };
        self.expect(Token::AssignmentOp(String::from("=")));
        let value: ASTNode = self.expression();
        ASTNode {
            node_type: ASTNodeType::VarDef(name, Box::from(value))
        }
    }

    pub fn command(&mut self, s: String) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Argslist
        }
    }

    pub fn if_statement(&mut self) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Argslist
        }
    }

    pub fn fn_def(&mut self) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Argslist
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test_1() {
        let p = Parser::new();
        assert_eq!(p.lookahead, Token::None);
    }
}