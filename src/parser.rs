use logos::Logos;

use crate::regex::Regex;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("^*")]
    #[token("*")]
    Star,
    #[token("+")]
    #[token("|")]
    Pipe,
    #[token("#")]
    EmptySet,
    #[token("$")]
    EmptyStr,
    #[regex(r"[^()|+*#$^ \t\n\r]", |lex| lex.slice().chars().next().unwrap())]
    Char(char),
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
}

pub struct ParseError {
    pub message: String,
    pub pos: usize,
}

struct TokenInfo {
    token: Token,
    start: usize,
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    pos: usize,
    input_len: usize,
}

impl Parser {
    pub fn parse(input: &str) -> Result<Regex, ParseError> {
        let mut tokens = Vec::new();
        let mut lex = Token::lexer(input);
        while let Some(result) = lex.next() {
            match result {
                Ok(token) => {
                    tokens.push(TokenInfo {
                        token,
                        start: lex.span().start,
                    });
                }
                Err(()) => {
                    let pos = lex.span().start;
                    let ch = input[pos..]
                        .chars()
                        .next()
                        .unwrap_or('?');
                    return Err(ParseError {
                        message: format!("无效字符 '{}'", ch),
                        pos,
                    });
                }
            }
        }
        let input_len = input.len();
        let mut parser = Parser {
            tokens,
            pos: 0,
            input_len,
        };
        let re = parser.parse_expression()?;
        if parser.pos != parser.tokens.len() {
            return Err(ParseError {
                message: "表达式末尾有多余字符".to_string(),
                pos: parser.current_pos(),
            });
        }
        Ok(re)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|t| &t.token)
    }

    fn current_pos(&self) -> usize {
        self.tokens
            .get(self.pos)
            .map_or(self.input_len, |t| t.start)
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].token.clone();
        self.pos += 1;
        token
    }

    fn parse_expression(&mut self) -> Result<Regex, ParseError> {
        let mut left = self.parse_term()?;
        while self.peek() == Some(&Token::Pipe) {
            self.advance();
            let right = self.parse_term()?;
            left = Regex::Union(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Regex, ParseError> {
        let mut left = self.parse_factor()?;
        while self.peek().map_or(false, |t| {
            matches!(
                t,
                Token::Char(_) | Token::EmptySet | Token::EmptyStr | Token::LParen
            )
        }) {
            let right = self.parse_factor()?;
            left = Regex::Concat(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Regex, ParseError> {
        let mut expr = self.parse_atom()?;
        while self.peek() == Some(&Token::Star) {
            self.advance();
            expr = Regex::Star(Box::new(expr));
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Regex, ParseError> {
        match self.peek() {
            None => Err(ParseError {
                message: "意外的表达式结束".to_string(),
                pos: self.current_pos(),
            }),
            Some(Token::Char(c)) => {
                let c = *c;
                self.advance();
                Ok(Regex::Char(c))
            }
            Some(Token::EmptySet) => {
                self.advance();
                Ok(Regex::EmptySet)
            }
            Some(Token::EmptyStr) => {
                self.advance();
                Ok(Regex::EmptyStr)
            }
            Some(Token::LParen) => {
                let open_pos = self.current_pos();
                self.advance();
                let expr = self.parse_expression()?;
                if self.peek() != Some(&Token::RParen) {
                    return Err(ParseError {
                        message: "缺少右括号 ')'".to_string(),
                        pos: open_pos,
                    });
                }
                self.advance();
                Ok(expr)
            }
            Some(t) => Err(ParseError {
                message: format!("此处不应该出现 '{}'", token_label(t)),
                pos: self.current_pos(),
            }),
        }
    }
}

fn token_label(t: &Token) -> String {
    match t {
        Token::LParen => "(".to_string(),
        Token::RParen => ")".to_string(),
        Token::Star => "*".to_string(),
        Token::Pipe => "|".to_string(),
        Token::EmptySet => "#".to_string(),
        Token::EmptyStr => "$".to_string(),
        Token::Char(c) => c.to_string(),
        Token::Whitespace => " ".to_string(),
    }
}
