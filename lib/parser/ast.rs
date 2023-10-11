use crate::lexer::{Lexer, token::{Token, TokenKind}};

pub enum Operator {
    Add,
    Sub,
}

pub enum Node {
    Int(i32),
    UnaryOp {
        op: Operator,
        child: Box<Node>,
    },
    BinaryOp {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Self {
            lexer: Lexer::new(source),
            cur_token: Token::empty(),
        }
    }

    pub fn build_ast(&mut self) -> Node {
        self.cur_token = self.lexer.get_next_token();

        let node = self.expr();
        return node;
    }

    /// expr: INT ((PLUS | MINUS) INT)*
    pub fn expr(&mut self) -> Node {

        let mut node: Node;

        match self.cur_token.kind {
            TokenKind::Int(i) => {
                node = Node::Int(i);
                self.cur_token = self.lexer.get_next_token();
            },
            _ => panic!("Expected integer"),
        }

        while self.cur_token.kind == TokenKind::Plus || self.cur_token.kind == TokenKind::Minus {
            let op = match self.cur_token.kind {
                TokenKind::Plus => Operator::Add,
                TokenKind::Minus => Operator::Sub,
                _ => panic!("Invalid operator"),
            };

            self.cur_token = self.lexer.get_next_token();
            let rhs = self.expr();
            node = Node::BinaryOp {
                op,
                lhs: Box::new(node),
                rhs: Box::new(rhs),
            };
        }

        return node;
    }




    pub fn eval(&mut self, node: Node) -> i32 {
        match node {
            Node::Int(i) => i,
            Node::UnaryOp { op, child } => {
                let child = self.eval(*child);
                match op {
                    Operator::Add => child,
                    Operator::Sub => -child,
                }
            },
            Node::BinaryOp { op, lhs, rhs } => {
                let lhs_value = self.eval(*lhs);
                let rhs_value = self.eval(*rhs);
                match op {
                    Operator::Add => lhs_value + rhs_value,
                    Operator::Sub => lhs_value - rhs_value,
                }
            }
            
        }
    }


}