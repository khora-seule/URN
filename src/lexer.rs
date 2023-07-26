
use indexmap::IndexSet;
use crate::{
    token::{ Token, TokenType, RESERVED, OPERATORS },
    error::{ Position, Error, LexError },
};

macro_rules! cinc {
    ( $column:expr ) => {
        *$column += 1
    }
}

macro_rules! linc {
    ( $line:expr, $column:expr ) => {
        {
            *$line += 1;
            *$column = 0;
        }
    }
}

macro_rules! process_ws {
    ( $stream:expr, $w:expr, $line:expr, $column:expr ) => {
        match $w {
            '\n'    => linc!($line,$column),
            '\r'    => {
                match $stream.pop() {
                    Some('\n')  => linc!($line,$column),
                    Some(oops)  => {
                        linc!($line,$column);
                        $stream.push(oops);
                    }
                    None        => linc!($line,$column),
                }
            }
            _       => cinc!($column),
        }
    };
}

macro_rules! build_position {
    ( $line:expr, $col:expr, $length:expr ) => {
        Position::build( $line, $line, $col, $col + $length )
    }
}

macro_rules! build_lex_error {
    ( $line:expr, $col:expr, $expected:expr, $recieved:expr ) => {
        Error::LexError(LexError::build( build_position!($line, $col, 1 ), $expected, $recieved ))
    }
}

pub fn lex_file( file: &mut String, names: &mut IndexSet<(Box<str>,TokenType)> ) -> ( Vec<Token>, Option<Vec<Error>> ) {

    // Source Position
    let mut line : usize = 1;
    let mut column : usize = 0;

    let mut tokens = Vec::new();
    
    let mut errors = Vec::new();

    while !file.is_empty() {
        match scan_token( file , names, &mut line, &mut column ) {
            Ok(Some(t)) => tokens.push( t ),
            Ok(None) => (),
            Err(err) => errors.push(err),
        }
    }
    if errors.len() > 0 {
        ( tokens, Some(errors) )
    }
    else {
        ( tokens, None )
    }
}

pub fn scan_token( stream: &mut String, names: &mut IndexSet<(Box<str>,TokenType)>, line: &mut usize, column: &mut usize ) -> Result<Option<Token>,Error> {

    let Some(c) = stream.pop() else { return Ok(Some(Token::Eof)) };

    match c {
        '('     => { cinc!(column); Ok(Some(Token::LeftParen)) },
        ')'     => { cinc!(column); Ok(Some(Token::RightParen)) },
        '{'     => { cinc!(column); Ok(Some(Token::LeftBrace)) },
        '}'     => { cinc!(column); Ok(Some(Token::RightBrace)) },
        '['     => { cinc!(column); Ok(Some(Token::LeftBracket)) },
        ']'     => { cinc!(column); Ok(Some(Token::RightBracket)) },
        op if OPERATORS.contains(&op) => {
            let mut literal = String::new();
            literal.push(op);
            let mut failed = false;
            loop {
                match stream.pop() {
                    Some(mop) if OPERATORS.contains(&mop)     =>  {
                        cinc!(column);
                        literal.push(mop);
                    },
                    Some(b) => {
                        stream.push(b);
                        break;
                    }
                    None    =>  break,
                }
            }
            let ( index, _ ) = names.insert_full((literal.into_boxed_str(),TokenType::Operator));
            Ok(Some(Token::Operator(index)))
        }
        '\'' => {
            let mut literal = String::new();
            let mut failed = false;
            loop {
                match stream.pop() {
                    Some('\'')  =>  {
                        cinc!(column);
                        break;
                    }
                    Some(x)     =>  {
                        cinc!(column);
                        literal.push(x);
                    },
                    None        =>  {
                        failed = true;
                        break;
                    }, 
                }
            }
            if !failed {
                let ( index, _ ) = names.insert_full((literal.into_boxed_str(),TokenType::String1));
                Ok(Some(Token::String(index)))
            }
            else {
                Err( build_lex_error!( *line, *column, Token::String(usize::MAX), Token::Eof ) )
            }
        }
        '\"' => {
            let mut literal = String::new();
            let mut failed = false;
            loop {
                match stream.pop() {
                    Some('\"')  =>  {
                        cinc!(column);
                        break;
                    }
                    Some(x)     =>  {
                        cinc!(column);
                        literal.push(x);
                    },
                    None        =>  {
                        failed = true;
                        break;
                    }, 
                }
            }
            if !failed {
                let ( index, _ ) = names.insert_full((literal.into_boxed_str(),TokenType::String2));
                Ok(Some(Token::String(index)))
            }
            else {
                Err( build_lex_error!( *line, *column, Token::String(usize::MAX), Token::Eof ) )
            }
        }
        a if a.is_ascii_alphabetic() => {
            cinc!(column);            
            let mut literal = String::new();
            literal.push(a);
            loop {
                match stream.pop() {
                    Some(w) if w.is_whitespace() => {
                        process_ws!(stream,w,line,column);
                        break;
                    },
                    Some(b)  => {
                        if !(RESERVED.contains(&b)) {
                            cinc!(column);
                            literal.push(b);
                        }
                        else {
                            stream.push(b);
                            break;
                        }
                    }
                    None => break,
                }
            }
            let ( index, _ ) = names.insert_full((literal.into_boxed_str(),TokenType::Atom));
            Ok(Some(Token::Atom(index)))
        }
        w if w.is_whitespace() => {
            process_ws!(stream, w, line, column);
            Ok(None)
        }
        uc => {
            cinc!(column);
            Err( build_lex_error!( *line, *column, Token::Valid, Token::Uc(uc) ) )
        }
    }
}