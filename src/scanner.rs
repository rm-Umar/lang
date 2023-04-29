use Object::*;
use TokenType::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });
        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            '*' => self.add_token(Star),
            ';' => self.add_token(SemiColon),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => {
                return Err(format!(
                    "unrecognized character at line: {} {}",
                    c, self.line
                ))
            }
        }
        Ok(())
    }

    fn string(self: &mut Self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("Unterminated string at line: {}", self.line));
        }

        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literals(Str, Some(StrValue(value.to_string())));
        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_literals(token_type, None);
    }

    fn add_token_literals(self: &mut Self, token_type: TokenType, literal: Option<Object>) {
        let mut lex = "".to_string();
        let _lit = self.source[self.start..self.current]
            .chars()
            .map(|ch| lex.push(ch));

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: lex,
            literal: literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    SemiColon,
    Slash,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    Str,
    Number,

    // Keywords.
    Eof,
    Wapis,  // RETURN
    Likho,  // PRINT
    Khali,  // NIL
    Rakho,  // VAR
    Jabtak, // WHILE
    Kaam,   // FUNCTION
    Ghalat, // FALSE
    Sahi,   // TRUE
    Agar,   // IF
    Warna,  // ELSE
    Ya,     // OR
    Aur,    // AND
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    NumValue(i64),
    FValue(f64),
    StrValue(String),
    IdenValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_char_token() {
        let source = "( { * + - . , } ) ;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftBrace);
        assert_eq!(scanner.tokens[2].token_type, Star);
        assert_eq!(scanner.tokens[3].token_type, Plus);
        assert_eq!(scanner.tokens[4].token_type, Minus);
        assert_eq!(scanner.tokens[5].token_type, Dot);
        assert_eq!(scanner.tokens[6].token_type, Comma);
        assert_eq!(scanner.tokens[7].token_type, RightBrace);
        assert_eq!(scanner.tokens[8].token_type, RightParen);
        assert_eq!(scanner.tokens[9].token_type, SemiColon);
        assert_eq!(scanner.tokens[10].token_type, Eof);
    }

    #[test]
    fn test_two_char_token() {
        let source = "! != = == > >= < <=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, EqualEqual);
        assert_eq!(scanner.tokens[4].token_type, Greater);
        assert_eq!(scanner.tokens[5].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[6].token_type, Less);
        assert_eq!(scanner.tokens[7].token_type, LessEqual);
    }

    #[test]
    fn test_string_literals() {
        let source = "\"This is a string\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        assert_eq!(scanner.tokens[0].token_type, Str);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StrValue(val) => assert_eq!(val, "This is a string"),
            _ => panic!("Incorrect literal type"),
        }
    }
}
