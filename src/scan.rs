use std::str::Chars;
use std::iter::{Peekable, Enumerate};

#[derive(PartialEq, Debug)]
pub enum Radix {
    Binary,
    Hexadecimal,
    Decimal,
}

#[derive(PartialEq, Debug)]
pub enum Lexeme<'a> {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Identifier(&'a str),
    Number(&'a str, Radix),
}

impl<'a> Lexeme<'a> {
    pub fn len(&self) -> usize {
        match self {
            Lexeme::Identifier(s) | Lexeme::Number(s, _) => s.len(),
            _ => 1,
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    source: &'a str,
    lexeme: Lexeme<'a>,
    start: usize,
}

impl<'a> Token<'a> {
    fn new(source: &'a str, lexeme: Lexeme<'a>, start: usize) -> Token<'a> {
        debug_assert!(start < source.len());
        debug_assert!(start + lexeme.len() <= source.len());

        Token {
            source,
            lexeme,
            start,
        }
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn lexeme(&self) -> &Lexeme {
        &self.lexeme
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.start + self.lexeme.len()
    }
}

type SourceIter<'a> = Peekable<Enumerate<Chars<'a>>>;

pub fn tokenize<'a>(source: &'a str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut source_iter: SourceIter = source.chars().enumerate().peekable();

    while let Some(&(start_idx, start_ch)) = source_iter.peek() {
        let mut add = |lexeme: Lexeme<'a>| {
            tokens.push(Token::new(source, lexeme, start_idx));
        };
        let mut consume = |lexeme: Lexeme<'a>| {
            add(lexeme);
            source_iter.next();
        };

        match start_ch {
            '(' => consume(Lexeme::LeftParen),
            ')' => consume(Lexeme::RightParen),
            '[' => consume(Lexeme::LeftBracket),
            ']' => consume(Lexeme::RightBracket),
            'a'..='z' | 'A'..='Z' | '_' => add(consume_identifier(source, start_idx, &mut source_iter)),
            '0'..='9' => add(consume_number(source, start_idx, &mut source_iter)),
            '#' => consume_comment(&mut source_iter),
            '\r' | '\t' | '\n' | ' ' => {
                source_iter.next();
            }
            _ => panic!("unknown token: {}", start_ch),
        };
    };

    tokens
}

fn consume_identifier<'a>(source: &'a str, start_idx: usize, source_iter: &mut SourceIter) -> Lexeme<'a> {
    while let Some(&(idx, ch)) = source_iter.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => source_iter.next(),
            _ => return Lexeme::Identifier(&source[start_idx..idx]),
        };
    };

    Lexeme::Identifier(&source[start_idx..])
}

fn consume_number<'a>(source: &'a str, start_idx: usize, source_iter: &mut SourceIter) -> Lexeme<'a> {
    if let &(_, '1'..='9') = source_iter.peek().unwrap() {
        return finish_decimal_number(source, start_idx, source_iter, false);
    };

    // consume '0'
    source_iter.next();

    match source_iter.peek() {
        Some(&(_, 'b')) => {
            source_iter.next();
            finish_binary_number(source, start_idx, source_iter)
        }
        Some(&(_, '0'..='9')) | Some(&(_, '_')) | Some(&(_, '.')) =>
            finish_decimal_number(source, start_idx, source_iter, false),
        Some(&(_, 'x')) => {
            source_iter.next();
            finish_hexadecimal_number(source, start_idx, source_iter)
        }
        _ => Lexeme::Number("0", Radix::Decimal)
    }
}

fn finish_binary_number<'a>(source: &'a str, start_idx: usize, source_iter: &mut SourceIter) -> Lexeme<'a> {
    while let Some(&(idx, ch)) = source_iter.peek() {
        match ch {
            '0' | '1' | '_' => source_iter.next(),
            _ => return Lexeme::Number(&source[start_idx..idx], Radix::Binary),
        };
    };

    Lexeme::Number(&source[start_idx..], Radix::Binary)
}

fn finish_decimal_number<'a>(source: &'a str, start_idx: usize, source_iter: &mut SourceIter, in_frac: bool) -> Lexeme<'a> {
    while let Some(&(idx, ch)) = source_iter.peek() {
        match ch {
            '0'..='9' | '_' => source_iter.next(),
            '.' if !in_frac => {
                source_iter.next();
                return finish_decimal_number(source, start_idx, source_iter, true);
            }
            _ => return Lexeme::Number(&source[start_idx..idx], Radix::Decimal),
        };
    };

    Lexeme::Number(&source[start_idx..], Radix::Decimal)
}

fn finish_hexadecimal_number<'a>(source: &'a str, start_idx: usize, source_iter: &mut SourceIter) -> Lexeme<'a> {
    while let Some(&(idx, ch)) = source_iter.peek() {
        match ch {
            '0'..='9' | 'a'..='f' | 'A'..='F' | '_' => source_iter.next(),
            _ => return Lexeme::Number(&source[start_idx..idx], Radix::Hexadecimal),
        };
    };

    Lexeme::Number(&source[start_idx..], Radix::Hexadecimal)
}

fn consume_comment(source_iter: &mut SourceIter) {
    // consume '#'
    source_iter.next();

    while let Some((_, ch)) = source_iter.next() {
        if let '\n' = ch {
            break;
        }
    };
}