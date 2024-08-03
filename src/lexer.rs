use std::{iter::Peekable, str::Chars};

use itertools::Itertools;

/// Matches either one or two characters, and returns the token.
/// Changes state of iterator.
/// 
/// There are cases where we want to lex two-character sequences, but
///     need to look ahead one character to determine what we are looking at.
///
/// Example: !xxx <--- Unparsed characters
/// .        ^--- If we are here, we need to look ahead to see if we are at an
/// .             Inequality '!=', or just LogicalNot '!'. 
/// .
/// .        In this case, we would use this function like so:
/// .        match_two_or_one(iter, '=', LogicalNot, Inequality)
///               
fn match_two_or_one(iter: &mut Peekable<Chars>, 
                    second: char, 
                    if_not_match: TokenKind, 
                    if_match: TokenKind) -> Token {
    let first = iter.peek()
        .expect("The iterator should point to a valid char when this method is called.")
        .clone();

    // Consume the first character, move to the second
    iter.next();
    if let Some(&next_char) = iter.peek() {
        if next_char == second {
            iter.next();
            Token{kind: if_match, lexeme: format!("{}{}",first,second)}
        }
        else { Token{kind: if_not_match, lexeme: first.to_string()} }
    }
    else { Token{kind: if_not_match, lexeme: first.to_string()} }
}

pub fn tokenize(input: String) -> Vec<Token> { 
    let mut iter = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(c) = iter.peek() {
        match c {
            // Single-character tokens
            '$' => { tokens.push(Token{kind: TokenKind::Dollar, lexeme: c.to_string()});
                     iter.next(); }
            ';' => { tokens.push(Token{kind: TokenKind::Semicolon, lexeme: c.to_string()});
                     iter.next(); }
            '(' => { tokens.push(Token{kind: TokenKind::LParen, lexeme: c.to_string()});
                     iter.next(); }
            ')' => { tokens.push(Token{kind: TokenKind::RParen, lexeme: c.to_string()});
                     iter.next(); }
            '{' => { tokens.push(Token{kind: TokenKind::LCurly, lexeme: c.to_string()});
                     iter.next(); }
            '}' => { tokens.push(Token{kind: TokenKind::RCurly, lexeme: c.to_string()});
                     iter.next(); }
            '[' => { tokens.push(Token{kind: TokenKind::LSquare, lexeme: c.to_string()});
                     iter.next(); }
            ']' => { tokens.push(Token{kind: TokenKind::RSquare, lexeme: c.to_string()});
                     iter.next(); }

            // Double-character tokens
            '=' => { tokens.push(match_two_or_one(&mut iter, '=', 
                     TokenKind::Assign, TokenKind::Equality)); }
            '!' => { tokens.push(match_two_or_one(&mut iter, '=', 
                     TokenKind::LogicalNot, TokenKind::Inequality)); }
            '|' => { tokens.push(match_two_or_one(&mut iter, '|', 
                     TokenKind::Pipe, TokenKind::LogicalOr)); }
            '&' => { tokens.push(match_two_or_one(&mut iter, '&', 
                     TokenKind::Ampersand, TokenKind::LogicalAnd)); }
            '>' => { tokens.push(match_two_or_one(&mut iter, '>', 
                     TokenKind::Redirect, TokenKind::CatRedirect)); }

            // Words
            c if c.is_alphanumeric() => {
                let lexeme: String = iter
                    .by_ref()
                    .peeking_take_while(|&x| x.is_alphanumeric())
                    .collect();

                // Keywords
                let kind = match lexeme.as_str() {
                    "while"  => {TokenKind::While}
                    "for"    => {TokenKind::For}
                    "if"     => {TokenKind::If}
                    "elif"   => {TokenKind::Elif}
                    "else"   => {TokenKind::Else}
                    _ => {TokenKind::Word}
                };

                tokens.push(Token{kind, lexeme});
                
            }

            // Strings
            '\'' | '"' => {
                let ch = c.clone();

                iter.next();
                let word = iter
                    .by_ref()
                    .take_while(|&x| x != ch)
                    .collect();

                match ch {
                    '\'' => { tokens.push(Token{kind: TokenKind::OneQuoteStr, 
                                                lexeme: word})}
                    '"'  => { tokens.push(Token{kind: TokenKind::TwoQuoteStr, 
                                                lexeme: format!("\"{}\"", word) })}
                    _ => {}
                }
            }
                

            // It might be useful if we separate this case from other whitespace
            '\n'=> { tokens.push(Token{kind: TokenKind::Newline, lexeme: "\\n".to_string()});
                     iter.next(); }

            // Skip whitespace
            c if c.is_whitespace() => {
                iter.next();
            }

            // Unrecognized
            _ => { tokens.push(Token { kind: TokenKind::Unknown, lexeme: c.to_string()});
                     iter.next(); }
        }
    }

    tokens
}

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String
}

#[derive(Debug)]
pub enum TokenKind {
    // Syntax
    Word,
    Semicolon,
    Ampersand,
    Dollar,
    Assign,
    OneQuoteStr,  // no interpol,  'hello world'
    TwoQuoteStr,  // yes interpol, "hello ${planet}"

    // Dataflow
    Pipe,
    Redirect,

    // Logical
    Equality,
    Inequality,
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    CatRedirect,
    
    // Parentheses
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,

    // Types
    TypeInt,
    TypeLong,
    TypeChar,
    TypeFloat,
    TypeDouble,

    // Etc
    Newline,
    Unknown,
    While,
    For,
    If,
    Elif,
    Else,
}
