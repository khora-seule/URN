
use std::collections::HashSet;

enum Token<'a> {
    Start,
    Stop,
    Word(&'a str),
}

#[derive(PartialEq, Eq)]
pub enum Term<'a> {
    Bound,
    Name(&'a str),
    Sentence(Box<[Term<'a>]>),
}

pub fn parse_files<'a>( term_file: &'a String, rule_file: &'a String ) -> ( Box<[Term<'a>]>, Box<[Term<'a>]>, Box<[&'a str]> ) {
    let (terms, term_names) = parse_terms( term_file );
    let (rules, rule_names) = parse_terms( rule_file );
    let names = term_names
        .union(&rule_names)
        .into_iter()
        .map(|x| *x )
        .collect::<Vec<_>>();
    ( terms, rules, (names).into_boxed_slice() )
}

fn parse_terms( term_file: &String ) -> ( Box<[Term]>, HashSet<&str> ) {
    let token_stream = lex_file( term_file );
    let tokens = token_stream.into_iter();

    let mut stack = Vec::new();

    let mut names = HashSet::new();

    for token in tokens {
        match token {
            Token::Start => stack.push(Term::Bound),
            Token::Word(word) => {
                stack.push(Term::Name(word));
                names.insert(*word);
            }
            Token::Stop => {

                let mut sentence : Vec<Term<'_>> = Vec::new();
                loop {
                    match stack.pop() {
                        Some(Term::Bound) => {
                            stack.push(Term::Sentence(sentence.into_boxed_slice()));
                            break
                        }
                        Some(x) => sentence.push(x),
                        None => break
                    }
                }                
            }
        }
    }

    ( stack.into_boxed_slice(), names )
}

fn lex_file( file: &String ) -> Box<[Token]> {
    file.as_str()
        .split_whitespace()
        .map_while(|x| Some(lex(x)) )
        .into_iter()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn lex( token: &str ) -> Token {

    match token {
        "{" => Token::Start,
        "}" => Token::Stop,
        _ => Token::Word( token ),
    }

}