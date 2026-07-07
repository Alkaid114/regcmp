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

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(input: &str) -> Result<Regex, String> {
        let mut tokens = Vec::new();
        let mut lex = Token::lexer(input);
        while let Some(result) = lex.next() {
            match result {
                Ok(token) => tokens.push(token),
                Err(()) => {
                    return Err(format!("无效字符位于位置 {}", lex.span().start));
                }
            }
        }
        let mut parser = Parser { tokens, pos: 0 };
        let re = parser.parse_expression()?;
        if parser.pos != parser.tokens.len() {
            return Err("表达式末尾有多余字符".to_string());
        }
        Ok(re)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    fn parse_expression(&mut self) -> Result<Regex, String> {
        let mut left = self.parse_term()?;
        while self.peek() == Some(&Token::Pipe) {
            self.advance();
            let right = self.parse_term()?;
            left = Regex::Union(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Regex, String> {
        let mut left = self.parse_factor()?;
        while self
            .peek()
            .map_or(false, |t| matches!(t, Token::Char(_) | Token::EmptySet | Token::EmptyStr | Token::LParen))
        {
            let right = self.parse_factor()?;
            left = Regex::Concat(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Regex, String> {
        let mut expr = self.parse_atom()?;
        while self.peek() == Some(&Token::Star) {
            self.advance();
            expr = Regex::Star(Box::new(expr));
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Regex, String> {
        match self.peek() {
            None => Err("意外的表达式结束".to_string()),
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
                self.advance();
                let expr = self.parse_expression()?;
                if self.peek() != Some(&Token::RParen) {
                    return Err("缺少右括号 ')'".to_string());
                }
                self.advance();
                Ok(expr)
            }
            _ => Err(format!("意外的token: {:?}", self.peek())),
        }
    }
}
