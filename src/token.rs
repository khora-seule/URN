//#![allow(unused)]

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash )]
pub enum Token {

    // Boundary Tokens
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]

    // Operator Tokens
    Operator(usize),
    Atom(usize),
    String(usize),

    // End of File
    Eof,

    Valid,
    Uc(char),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy )]
pub enum TokenType {
    Atom,
    Operator,
    String1,
    String2,
}

lazy_static! {
    pub static ref RESERVED : HashSet::<char> = HashSet::from([     
    '(', ')', '{','}','[', ']', '`', '~', '!', '@', '#', '$', '%', 
    '^', '&', '*', '-', '_', '=', '+', '\\', '|', ';', ':', ',', 
    '<', '.', '>', '/', '?', '\'', '\"',
    ]);
}


lazy_static! {
    pub static ref OPERATORS : HashSet::<char> = HashSet::from([     
    '`', '~', '!', '@', '#', '$', '%', '^', '&', '*', '-', 
    '_', '=', '+', '\\', '|', ';', ':', ',', '<', '.', 
    '>', '/', '?',
    ]);
}