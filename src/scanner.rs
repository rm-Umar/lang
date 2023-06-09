use std::collections::HashMap;
use Object::*;
use TokenType::*;

fn get_keywords() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("wapis", Wapis),   // RETURN
        ("likho", Likho),   // PRINT
        ("khali", Khali),   // NIL
        ("maanlo", Maanlo), // VAR
        ("jabtak", Jabtak), // WHILE
        ("kaam", Kaam),     // FUNCTION
        ("ghalat", Ghalat), // FALSE
        ("sahi", Sahi),     // TRUE
        ("agar", Agar),     // IF
        ("warna", Warna),   // ELSE
        ("ya", Ya),         // OR
        ("aur", Aur),       // AND
    ])
}
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keywords(),
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
            c => {
                if is_digit(c) {
                    self.number()?;
                } else if is_alpha(c) {
                    self.identifier()?;
                } else {
                    return Err(format!(
                        "unrecognized character at line: {} {}",
                        c, self.line
                    ));
                }
            }
        }
        Ok(())
    }

    fn identifier(self: &mut Self) -> Result<(), String> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        if let Some(&ref token_type) = self.keywords.get(text) {
            self.add_token(token_type.clone());
        } else {
            self.add_token(Identifier);
        }
        Ok(())
    }

    fn number(self: &mut Self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token_literal(Number, Some(FValue(value)));
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
        self.add_token_literal(Str, Some(StrValue(value.to_string())));
        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(self: &Self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(self: &mut Self, token_type: TokenType, literal: Option<Object>) {
        let lex = self.source[self.start..self.current].chars().collect();
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
    Maanlo, // VAR
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
    FValue(f64),
    StrValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line_number: usize,
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

fn is_digit(ch: char) -> bool {
    ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8
}

fn is_alpha(ch: char) -> bool {
    (ch as u8 >= 'a' as u8 && ch as u8 <= 'z' as u8)
        || (ch as u8 >= 'A' as u8 && ch as u8 <= 'Z' as u8)
        || ch as u8 == '_' as u8
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_char_token() {
        let source = "( { * + - . , } ) ;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("failed to scan tokens");
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
        scanner.scan_tokens().expect("failed to scan tokens");
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
    fn test_string_literal() {
        let source = "\"This is a string\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("failed to scan tokens");
        assert_eq!(scanner.tokens[0].token_type, Str);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StrValue(val) => assert_eq!(val, "This is a string"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn test_number_literal() {
        let source = "123.12 \n 5 \n 0.06";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("failed to scan tokens");
        assert_eq!(scanner.tokens[0].token_type, Number);
        assert_eq!(scanner.tokens[1].token_type, Number);
        assert_eq!(scanner.tokens[2].token_type, Number);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 123.12),
            _ => panic!("Incorrect literal type"),
        }

        match scanner.tokens[1].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 5.0),
            _ => panic!("Incorrect literal type"),
        }

        match scanner.tokens[2].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 0.06),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn test_identifier() {
        let source = "naam = \"Ali\"; \n jamaat = 8;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("failed to scan tokens");
        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[1].token_type, Equal);
        assert_eq!(scanner.tokens[2].token_type, Str);
        assert_eq!(scanner.tokens[3].token_type, SemiColon);
        assert_eq!(scanner.tokens[4].token_type, Identifier);
        assert_eq!(scanner.tokens[5].token_type, Equal);
        assert_eq!(scanner.tokens[6].token_type, Number);
        assert_eq!(scanner.tokens[7].token_type, SemiColon);
    }

    #[test]
    fn test_reserved_keywords() {
        let source = "jabtak agar warna likho wapis ghalat sahi khali kaam maanlo aur";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("failed to scan tokens");
        assert_eq!(scanner.tokens[0].token_type, Jabtak);
        assert_eq!(scanner.tokens[1].token_type, Agar);
        assert_eq!(scanner.tokens[2].token_type, Warna);
        assert_eq!(scanner.tokens[3].token_type, Likho);
        assert_eq!(scanner.tokens[4].token_type, Wapis);
        assert_eq!(scanner.tokens[5].token_type, Ghalat);
        assert_eq!(scanner.tokens[6].token_type, Sahi);
        assert_eq!(scanner.tokens[7].token_type, Khali);
        assert_eq!(scanner.tokens[8].token_type, Kaam);
        assert_eq!(scanner.tokens[9].token_type, Maanlo);
        assert_eq!(scanner.tokens[10].token_type, Aur);
    }
}
