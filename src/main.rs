
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

    let term_table = TermTable::build_term_table(parsed_terms,parsed_rules,names);


    println!("{:?}", term_table);
}
