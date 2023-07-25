
use crate::token::Token;

#[derive(Debug)]
pub struct Position {
    pub line_start: usize,
    pub line_stop: usize,
    pub col_start: usize,
    pub col_stop: usize,
}
impl Position {
    pub fn build( line_start: usize, line_stop: usize, col_start: usize, col_stop: usize ) -> Self {
        Position { line_start, line_stop, col_start, col_stop }
    }
}
pub enum Error{
    LexError(LexError),
    ParseError(ParseError),
}

#[derive(Debug)]
pub struct LexError {
    pub pos: Position,
    exp_token:  Token,
    err_token:  Token,
}
impl LexError {
    pub fn build( pos: Position, exp_token: Token, err_token: Token ) -> Self {
        LexError { pos, exp_token, err_token }
    }
}

pub struct ParseError;


/*
struct ParseError {
    pos: Position,
}
impl LexError {
    fn build( pos: Position, ) -> Self {
        todo!()
    }
}
*/