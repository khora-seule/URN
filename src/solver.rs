use crate::parser::Term;
use std::collections::HashSet;
use indexmap::IndexSet;

#[derive(PartialEq, Eq, Hash, Debug)]
enum TermIR {
    Name(usize),
    Term(usize),
}

#[derive(Debug)]
struct TermSet(IndexSet<Box<[TermIR]>>);

impl TermSet {
    fn insert( &mut self, names: &Box<[&str]>, parsed_term: &[Term] ) -> usize {

        let mut new_term = Vec::new();

        for sub_term in parsed_term.into_iter() {
            match sub_term {
                Term::Name(name) => new_term.push(TermIR::Name(names.iter().position(|x| x == name).unwrap() )),
                Term::Sentence(ptr) => new_term.push(TermIR::Term(self.insert( names, &*ptr ))),
                Term::Bound => todo!(),
            }
        }
        let (index,_) = self.0.insert_full(new_term.into_boxed_slice());
        index
    }
}


#[derive(Debug)]
pub struct TermTable<'a> {
    names: Box<[&'a str]>,
    terms: TermSet,
    term_indices: HashSet<usize>,
    rule_indices: Box<[(usize,usize)]>
}
impl TermTable<'_> {
    pub fn build_term_table<'a>( parsed_terms: Box<[Term<'a>]>, parsed_rules: Box<[Term<'a>]>, names:  Box<[&'a str]> ) -> TermTable<'a>  {
        
        let mut terms = TermSet(IndexSet::new());
        let mut term_indices = HashSet::new();
        let mut rule_indices = IndexSet::new();

        for rule_pair in parsed_rules.chunks(2) {
            let pre_index;
            let post_index;
            match &rule_pair[0] {
                Term::Sentence(ptr) => {

                    pre_index = terms.insert(&names, &*ptr);
                }
                _ => todo!(),
            }
            match &rule_pair[1] {
                Term::Sentence(ptr) => {
                    post_index = terms.insert(&names, &*ptr);
                }
                _ => todo!(),
            }
            rule_indices.insert((pre_index,post_index));
        }

        for term in parsed_terms.into_iter() {
            let index;
            match term {
                Term::Sentence(ptr) => {
                    index = terms.insert(&names, &*ptr);
                }
                _ => todo!(),
            }

            term_indices.insert(index);
        }

        TermTable {
            names,
            terms: terms,
            term_indices: term_indices,
            rule_indices: rule_indices.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        }
    }
    // Returns a bool indicating whether or not a rewrite occured for all steps
    // True if so, false if no rewrite target was found in some step
    /*
    fn rewrite( &mut self,  steps: usize ) -> bool {
        let total = true;
        'pass: for step in 0..steps {
            for (pre_index,post_index) in self.rules_indices.into_iter() {
                let target = self.term_indices.into_iter().find( |x| x == pre_index );
                match target {
                    Some(_) => {

                    }
                    None => {
                        let total = false;
                        break 'pass
                    }
                    
                }
            }
        }
        total
    }
    */
}