#[macro_use]
extern crate lazy_static;

use std::{ collections::HashSet, env };
mod runtime;
mod error;
mod token;
mod lexer;
mod parser;
// mod solver;
use crate::runtime::{ run_file, run_repl };

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        // Check first argument for `.aux` file extension
        // If found, run said file with any flags
        if args[1].contains(".urn") & args[2].contains(".urn") {

            let flags = args[3..].iter().collect::<HashSet<_>>();
 
            run_file(&args[1], &args[2], Some(flags));
        }
        // If not found, then we launch the prompt with flags
        else {

            let flags = args[1..].iter().collect::<HashSet<_>>();

            run_repl(Some(flags));
        }
    }
    else {
        run_repl(None);
    }


}
