use crate::Constant;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Plus,
    Minus,
    Div,
    Multiply,
    Pow,
    LParem,
    RParem,
    LSquirly,
    RSquirly,
    Equal,
    GreaterThan,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,
    Eof,
    Constant(Constant),
    Symbol(String),
}

impl Token {
    pub fn is_binary(&self) -> bool {
        match self {
            Token::Plus => true,
            Token::Minus => true,
            Token::Multiply => true,
            Token::Div => true,
            Token::Pow => true,
            Token::Equal => true,
            Token::GreaterThan => true,
            Token::GreaterThanEqualTo => true,
            Token::LessThan => true,
            Token::LessThanEqualTo => true,
            _ => false,
        }
    }

    pub fn is_bracket(&self) -> bool {
        match self {
            Token::LSquirly => true,
            Token::RSquirly => true,
            Token::LParem => true,
            Token::RParem => true,
            _ => false,
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            Token::Constant(_) => true,
            _ => false,
        }
    }

    pub fn is_relational(&self) -> bool {
        match self {
            Token::Equal => true,
            Token::GreaterThan => true,
            Token::GreaterThanEqualTo => true,
            Token::LessThan => true,
            Token::LessThanEqualTo => true,
            _ => false,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Token::Symbol(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        return lex;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_number(&mut self) -> Constant {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let num = String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
        return Constant::Num(num);
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let tok = match self.ch {
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'(' => Token::LParem,
            b')' => Token::RParem,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Multiply,
            b'/' => Token::Div,
            b'=' => Token::Equal,
            b'^' => Token::Pow,
            b'>' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::GreaterThanEqualTo
                } else {
                    Token::GreaterThan
                }
            }
            b'<' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::LessThanEqualTo
                } else {
                    Token::LessThan
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Some(match ident.as_str() {
                    "e" => Token::Constant(Constant::Euler),
                    "pi" => Token::Constant(Constant::Pi),
                    _ => Token::Symbol(ident),
                });
            }
            b'0'..=b'9' => return Some(Token::Constant(self.read_number())),
            // end of file
            0 => return Some(Token::Eof),
            _ => return None,
        };

        self.read_char();
        return Some(tok);
    }
}
