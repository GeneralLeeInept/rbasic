#[derive(Debug)]
pub enum TokenKind {
    Bad,
    Eof,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Integer(i64),
}

#[derive(Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: Option<String>,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: Option<String>) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    text_span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, text_span: TextSpan) -> Self {
        Self { kind, text_span }
    }
}

pub struct Lexer<'a> {
    input: &'a String,
    input_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Lexer {
            input,
            input_pos: 0,
        }
    }

    pub fn get(&mut self) -> Option<Token> {
        loop {
            if self.input_pos > self.input.len() {
                return None;
            } else if let Some(c) = self.peek() {
                let start = self.input_pos;
                if c.is_whitespace() {
                    self.input_pos += 1;
                    continue;
                } else if c.is_ascii_digit() {
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            self.input_pos += 1;
                        } else {
                            break;
                        }
                    }
                    let literal = self.input.get(start..self.input_pos).unwrap();
                    let i: i64 = literal.parse().unwrap();
                    return Some(Token::new(
                        TokenKind::Integer(i),
                        TextSpan::new(start, self.input_pos, Some(literal.to_string())),
                    ));
                } else {
                    self.input_pos = self.input.len();
                    return Some(Token::new(
                        TokenKind::Bad,
                        TextSpan::new(
                            start,
                            self.input_pos,
                            self.input.get(start..self.input_pos).map(str::to_string),
                        ),
                    ));
                }
            } else {
                self.input_pos += 1;
                return Some(Token::new(TokenKind::Eof, TextSpan::new(0, 0, None)));
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.chars().nth(self.input_pos)
    }
}
