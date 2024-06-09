use std::fmt::Display;

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
impl Into<String> for &TokenType {
    fn into(self) -> String {
        match self {
            // single char
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::LeftBrace => "{".to_string(),
            TokenType::RightBrace => "}".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::Star => "*".to_string(),

            // one or two chars
            TokenType::Bang => "!".to_string(),
            TokenType::BangEqual => "!=".to_string(),
            TokenType::Equal => "=".to_string(),
            TokenType::EqualEqual => "==".to_string(),
            TokenType::Greater => ">".to_string(),
            TokenType::GreaterEqual => ">=".to_string(),
            TokenType::Less => "<".to_string(),
            TokenType::LessEqual => "<=".to_string(),

            // literals
            TokenType::Identifier(t) => format!("Identifier({})", t),

            // values
            TokenType::String(s) => format!("String({})", s),
            TokenType::Number(n) => format!("Number({})", n),
            TokenType::Nil => "nil".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::False => "false".to_string(),

            // keywords
            TokenType::And => "and".to_string(),
            TokenType::Class => "class".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::Fn => "fn".to_string(),
            TokenType::For => "for".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Or => "or".to_string(),
            TokenType::Print => "print".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::This => "this".to_string(),
            TokenType::Var => "var".to_string(),
            TokenType::While => "while".to_string(),

            TokenType::Eof => "Eof".to_string(),
        }
    }
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token_str: String = self.into();
        write!(f, "{}", token_str)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
