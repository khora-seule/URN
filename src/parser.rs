
use indexmap::IndexSet;


use crate::token::Token;

lazy_static! {
    static ref OPERATORS : IndexSet::<Token> = IndexSet::from([ 
        Token::Tick, Token::Tilde, Token::Bang, Token::At, 
        Token::Hash, Token::Dollar, Token::Percent, Token::Caret,
        Token::Ampersand, Token::Asterisk, Token::Dash, Token::Underscore,
        Token::Equal, Token::Plus, Token::BSlash, Token::Vertical, 
        Token::Semicolon, Token::Colon, Token::Comma, Token::Left,
        Token::Dot, Token::Right, Token::FSlash, Token::Query,
    ]);
}

#[derive(PartialEq, Eq)]
pub enum Term {
    Bound(TermType),
    Word(usize),
    Oper(usize),
    Sentence(Box<[Term]>,TermType),
}

#[derive(PartialEq, Eq)]
enum TermType {
    Paren,
    Brace,
    Bracket,
}

pub fn parse_terms( tokens: &mut Vec<Token> ) -> Box<[Term]> {

    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::LeftParen => stack.push(Term::Bound(TermType::Paren)),
            Token::LeftBrace => stack.push(Term::Bound(TermType::Brace)),
            Token::LeftBracket => stack.push(Term::Bound(TermType::Bracket)),
            Token::RightParen => {
                let mut sentence = Vec::new();
                loop {
                    match stack.pop() {
                        Some(Term::Bound(TermType::Paren)) => {
                            stack.push(Term::Sentence(sentence.into_boxed_slice(),TermType::Paren));
                            break
                        }
                        Some(other) => sentence.push(other),
                        None => break,
                    }
                }
            }
            Token::RightBrace => {
                let mut sentence = Vec::new();
                loop {
                    match stack.pop() {
                        Some(Term::Bound(TermType::Brace)) => {
                            stack.push(Term::Sentence(sentence.into_boxed_slice(),TermType::Brace));
                            break
                        }
                        Some(other) => sentence.push(other),
                        None => break,
                    }
                }
            },
            Token::RightBracket => {
                let mut sentence = Vec::new();
                loop {
                    match stack.pop() {
                        Some(Term::Bound(TermType::Bracket)) => {
                            stack.push(Term::Sentence(sentence.into_boxed_slice(),TermType::Bracket));
                            break
                        }
                        Some(other) => sentence.push(other),
                        None => break,
                    }
                }
            },
            
            Token::Atom(id)     => stack.push(Term::Word(*id)),
            Token::String(id)  => stack.push(Term::Word(*id)),
            Token::Eof  => todo!(),
            _ => todo!()
        }
    }
    stack.into_boxed_slice()
}
