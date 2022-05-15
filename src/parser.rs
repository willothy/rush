use std::boxed::Box;
use std::fmt;

use crate::tokenizer::Tokenizer;
use crate::tokenizer::token::Token;

//pub use crate::parser::SyntaxError::*;

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

#[derive(Debug, PartialEq)]
pub enum ASTNodeType {
    StatementList(Vec<ASTNode>),
    Statement(Box<ASTNode>),
    Expression(Box<ASTNode>),
    VarDef(String, Box<ASTNode>),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64),
    BoolLiteral(bool),
    Command(String, Vec<ASTNode>),
    Argslist,
    None
}

#[derive(Debug, PartialEq)]
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
            lookahead: Token::Empty,
            tokenizer: Tokenizer::new()
        }
    }

    pub fn parse(&mut self, input: &str) -> ASTNode {
        self.tokenizer.init(input);
        self.update_lookahead();
        return ASTNode {
            node_type: ASTNodeType::StatementList(self.statement_list())
        }
    }

    fn update_lookahead(&mut self) {
        self.lookahead = self.tokenizer.get_next_token(false);
    }

    fn expect(&mut self, tok_type: Token) -> Token {
        let token = self.lookahead.clone();

        if token == tok_type {
            self.update_lookahead();
            return token
        }
        panic!("{}", SyntaxError::UnexpectedToken(token.to_string()));
    }

    /**
     *  StatementList
     *  : Statement
     *  | Statement LOGICAL_OP StatementList
     */
    pub fn statement_list(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::<ASTNode>::new();

        while self.lookahead != Token::Empty {
            println!("{}", self.lookahead.clone());
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
        let look = self.lookahead.clone();
        match look {
            Token::Keyword(s) if s == String::from("if")
                => self.if_statement(),
            Token::Keyword(s) if s == String::from("fn")
                => self.fn_def(),
            Token::Let
                => self.var_def(),
            Token::Identifier(s)
                => self.command_expression(),
            _ => panic!(),
        }
    }

    pub fn command_expression(&mut self) -> ASTNode {
        let command = self.expect(Token::Identifier(String::new()));
        let mut args = Vec::<ASTNode>::new();
        loop {
            match self.lookahead.clone() {
                Token::Identifier(s) => {
                    let ident_str = match self.expect(Token::Identifier(String::new())) {
                        Token::Identifier(s) => s,
                        _ => panic!()
                    };
                    args.push(
                        self.identifier(
                            ident_str
                        )
                    )
                },
                Token::NumberLiteral(n) => args.push(self.number_literal(n)),
                Token::StringLiteral(s) => args.push(self.string_literal(s)),
                Token::BoolLiteral(b) => args.push(self.bool_literal(b)),
                _ => break
            }
        }
        ASTNode {
            node_type: ASTNodeType::None
        }
    }

    pub fn identifier(&mut self, s: String) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::Identifier(s)
        }
    }

    pub fn number_literal(&mut self, n: f64) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::NumberLiteral(n)
        }
    }

    pub fn string_literal(&mut self, s: String) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::StringLiteral(s)
        }
    }

    pub fn bool_literal(&mut self, b: bool) -> ASTNode {
        ASTNode {
            node_type: ASTNodeType::BoolLiteral(b)
        }
    }

    /**
     *  Expression
     *  : BinaryExpression
     *  | Command Argslist
     */
    pub fn expression(&mut self) -> ASTNode {
        let look = self.lookahead.clone();
        match look {
            Token::StringLiteral(string) => match self.expect(Token::StringLiteral(string)) {
                Token::StringLiteral(s) => ASTNode {
                    node_type: ASTNodeType::StringLiteral(s)
                },
                _ => panic!()
            },
            Token::NumberLiteral(number) => match self.expect(Token::NumberLiteral(number)) {
                Token::NumberLiteral(n) => ASTNode {
                    node_type: ASTNodeType::NumberLiteral(n)
                },
                _ => panic!()
            },
            Token::BoolLiteral(boolean) => match self.expect(Token::BoolLiteral(boolean)) {
                Token::BoolLiteral(b) => ASTNode {
                    node_type: ASTNodeType::BoolLiteral(b)
                },
                _ => panic!()
            },
            Token::OpenParen => match self.expect(Token::OpenParen) {
                Token::OpenParen => self.parenthesized_expression(),
                _ => panic!()
            }
            _ => panic!()
        }
    }

    pub fn parenthesized_expression(&mut self) -> ASTNode {
        self.expect(Token::OpenParen);
        let node = ASTNode {
            node_type: ASTNodeType::Expression(Box::from(self.expression()))
        };
        self.expect(Token::CloseParen);
        node
    }

    pub fn var_def(&mut self) -> ASTNode {
        self.expect(Token::Let);
        let name: String = match self.expect(Token::Identifier(String::new())) {
            Token::Identifier(name) => name,
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
            node_type: ASTNodeType::Command(String::from(""), Vec::new())
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
        assert_eq!(p.lookahead, Token::Empty);
    }

    #[test]
    fn parser_test_2() {
        let mut p = Parser::new();
        let parsed = p.parse("let xawd = 10.0");
        println!("{:?}", parsed);
        let mut control = Vec::new();
        control.push(
            ASTNode{
                node_type: ASTNodeType::VarDef(String::from("xawd"), Box::from(
                    ASTNode {
                        node_type: ASTNodeType::NumberLiteral(10.0)
                    }
                )),
            }
        );
        assert_eq!(parsed, ASTNode {
            node_type: ASTNodeType::StatementList(control)
        });
    }
}