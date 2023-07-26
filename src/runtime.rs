use std::{ collections::HashSet, io, fs::read_to_string };

use indexmap::IndexSet;

use crate::lexer::lex_file;
use crate::parser::parse_terms;
use crate::solver::TermTable;

pub fn run_file( term_path: &String, rule_path: &String, flags: Option<HashSet<&String>> ) {
    match read_to_string( term_path ) {
        Err(e) => println!("{}",e),
        Ok(mut term_source) => {
            match read_to_string( rule_path ) {
                Err(e) => println!("{}",e),
                Ok( mut rule_source ) => run( &mut term_source, &mut rule_source ),
            }
        }
    }
}

pub fn run_repl( pre_flags: Option<HashSet<&String>> ) {


    let stdin = io::stdin();
    let line = &mut String::new();

    match pre_flags {
        Some(flags) => { todo!() }
        None =>     {
            loop {
                todo!();
                /*
                print!("> ");
                let Ok(_) = stdin.read_line(line) else { todo!() };
                run( &mut line.chars().rev().collect::<String>());
                */
            }
        }
    }


}

fn run( term_source: &mut String, rule_source: &mut String, ) {
    
    let mut interned_names = IndexSet::new();

    let ( mut lexed_rules, _) = lex_file(  &mut rule_source.chars().rev().collect::<String>(), &mut interned_names );
    let ( mut lexed_terms, _) = lex_file(  &mut term_source.chars().rev().collect::<String>(), &mut interned_names );

    let names = interned_names.into_iter().collect::<Vec<_>>().into_boxed_slice();

    //println!("{:?}",lexed_rules);
    //println!("{:?}",lexed_terms);

    let parsed_rules = parse_terms( &mut lexed_rules );
    let parsed_terms = parse_terms( &mut lexed_terms );


    let mut term_table = TermTable::build( parsed_terms, parsed_rules, names );

    //println!("{:?}",parsed_rules);
    //println!("{:?}",parsed_terms);

    let display = term_table.display();

    if display.len() == 1 {
        println!("|~| {}", display.iter().next().unwrap());
    }
    else {
        for (index, term) in display.iter().enumerate() {
            println!("|{}|\t{}",index+1,term);
        }
    }
        
    let mut steps = 1;

    while term_table.rewrite() {
        let display = term_table.display();
        if display.len() == 1 {
            println!("|~| {}", display.iter().next().unwrap());
        }
        else {
            for (index, term) in display.iter().enumerate() {
                println!("|{}|\t{}",index+1,term);
            }
        }
    }
}

