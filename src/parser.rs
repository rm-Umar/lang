use crate::scanner::{ TokenType::*, Token };
use crate::expression::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut exp = self.comparison();

        while self.match_tokens(BangEqual, EqualEqual) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Binary {
                left: expr,
                operator: operator,
                right: rhs,
            };

            expr
        }
    }
}
