#[derive(Clone, Debug)]
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One Or Two Character Tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier(String), String(String), Number(f64),

  // Keywords.
  And, Class, Else, False, Fn, For, If, Nil, Or,
  Print, Return, This, True, Var, While,

  Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
