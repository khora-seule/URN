use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
mod parser;
mod solver;
use crate::parser::parse_files;
use crate::solver::{ TermTable };

fn main() {

    let args: Vec<String> = env::args().collect();

    let terms_path = &args[1];
    let rules_path = &args[2];
    let terms_file = read_to_string( terms_path ).unwrap();
    let rules_file = read_to_string( rules_path ).unwrap();

    let (parsed_terms,parsed_rules,names) = parse_files(&terms_file,&rules_file);

    let mut term_table = TermTable::build_term_table(parsed_terms,parsed_rules,names);

    let flags = &args[2..].iter().collect::<HashSet<_>>();

    let verbose = flags.contains(&String::from("-v"));

    let spacious = flags.contains(&String::from("-s"));

    let raw = flags.contains(&String::from("-r"));

    if raw {
        for term in term_table.display().iter() {
            println!("{}",term);
        }
        while term_table.rewrite() {
            for term in term_table.display().iter() {
                println!("{}",term);
            }
        }
    }
    else {

        if verbose {
            println!("Initial Term(s):");
        }

        let display = term_table.display();

        if display.len() == 1 {
            println!("|~|{}", display.iter().next().unwrap());
        }
        else {
            for (index, term) in display.iter().enumerate() {
                println!("|{}|\t{}",index+1,term);
            }
        }
        
        if spacious {
            println!("|~|");
        }

        let mut steps = 1;

        while term_table.rewrite() {
            let display = term_table.display();
            if verbose {
                println!("|~| Number of Rewrites: {}\n|~|\n|~| Current Term(s)", steps);
                steps += 1;
            }
            if display.len() == 1 {
                println!("|~| {}", display.iter().next().unwrap());
            }
            else {
                for (index, term) in display.iter().enumerate() {
                    println!("|{}\t{}",index+1,term);
                }
            }
            if spacious {
                println!("|~|");
            }    
        }
    }

}
