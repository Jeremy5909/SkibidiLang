use std::fmt::Display;

use crate::{convert, token::TokenType};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expr>
    }

}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => write!(f, "({operator} ({left}) ({right}))"),
            Expr::Grouping { expression } => write!(f, "group {expression}"),
            Expr::Literal { value } => write!(f, "{value}"),
            Expr::Unary { operator, right } => write!(f, "{operator} {right}",),
        }
    }
}




#[derive(Debug)]
pub enum BinaryOperator {
    Slash,
    Star,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}
impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token: TokenType = self.into();
        write!(f, "{token}")
    }
}
impl TryFrom<TokenType> for BinaryOperator {
    type Error = ();

    fn try_from(token: TokenType) -> Result<Self, ()> {
        convert!(TokenType; BinaryOperator; token; Slash,Star,BangEqual,Equal,EqualEqual,Greater,GreaterEqual,Less,LessEqual,;)
    }
}
impl Into<TokenType> for &BinaryOperator {
    fn into(self) -> TokenType {
        let token = convert!(BinaryOperator; TokenType; self; Slash,Star,BangEqual,Equal,EqualEqual,Greater,GreaterEqual,Less,LessEqual,;);
        token.unwrap()
    }
}




#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
    Nil,
    True,
    False,
}
impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token: TokenType = self.into();
        write!(f, "{token}")
    }
}
impl Into<TokenType> for &Literal {
    fn into(self) -> TokenType {
        let token = convert!(Literal; TokenType; self; Nil,True,False,;Identifier,Number,String,);
        token.unwrap()
    }
}

impl TryFrom<TokenType> for Literal {
    type Error = ();
    fn try_from(token: TokenType) -> Result<Literal, ()> {
        convert!(TokenType; Literal; token; Nil,True,False,;Identifier,Number,String,)
    }
}




#[derive(Debug)]
pub enum UnaryOperator {
    Minus,
    Bang,
}
impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token: TokenType = self.into();
        write!(f, "{token}")
    }
}
impl TryFrom<TokenType> for UnaryOperator {
    type Error = ();
    fn try_from(token: TokenType) -> Result<UnaryOperator, ()> {
        convert!(TokenType; UnaryOperator; token; Minus,Bang,;)
    }
}
impl Into<TokenType> for &UnaryOperator {
    fn into(self) -> TokenType {
        let token = convert!(UnaryOperator; TokenType; self; Minus,Bang,;);
        token.unwrap()
    }
}
