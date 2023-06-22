use crate::parser::Term;
use std::collections::HashSet;
use indexmap::IndexSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TermIR {
    Name(usize),
    Term(usize),
}

#[derive(Debug)]
struct TermSet(IndexSet<Box<[TermIR]>>);

impl TermSet {
    fn insert_raw( &mut self, names: &Box<[&str]>, parsed_term: &[Term] ) -> usize {

        let mut new_term = Vec::new();

        for sub_term in parsed_term.into_iter() {
            match sub_term {
                Term::Name(name) => new_term.push(TermIR::Name(names.iter().position(|x| x == name).unwrap() )),
                Term::Sentence(ptr) => new_term.push(TermIR::Term(self.insert_raw( names, &*ptr ))),
                Term::Bound => todo!(),
            }
        }
        self.insert(new_term.into_boxed_slice())
    }
    fn insert( &mut self, term: Box<[TermIR]> ) -> usize {
        let (index,_) = self.0.insert_full(term);
        index
    }
    fn get_index( &mut self, index: usize ) -> &Box<[TermIR]> {
        self.0.get_index( index ).unwrap()
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

                    pre_index = terms.insert_raw(&names, &*ptr);
                }
                _ => todo!(),
            }
            match &rule_pair[1] {
                Term::Sentence(ptr) => {
                    post_index = terms.insert_raw(&names, &*ptr);
                }
                _ => todo!(),
            }
            rule_indices.insert((pre_index,post_index));
        }

        for term in parsed_terms.into_iter() {
            let index;
            match term {
                Term::Sentence(ptr) => {
                    index = terms.insert_raw(&names, &*ptr);
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

    // Returns a bool, usize pair indicating whether or not a rewrite occured for all steps
    // True if so, false if no rewrite target was found and at which step
    pub fn rewrite( &mut self,  steps: usize ) -> ( bool, usize ) {
        let mut new_top_term_index: usize;
        let external = false;
        let internal = false;

        'pass: for step in 0..steps {
            for (pre_index,post_index) in self.rule_indices.into_iter() {
                let mut top_level = false;
                // First see if any top level terms match the Rule
                let target = self.term_indices.take( pre_index );

                match target {

                    // If they do insert the index for the post-term of the rule into term-indices
                    Some(_) => {
                        new_top_term_index = *post_index;
                        let external = true;
                        top_level = true;
                    }
                    None => {
                        new_top_term_index = 0;
                    },
                }

                // Then check if any of the current terms have subterms that match the rule
                for index in self.term_indices.clone().iter() {
                    
                    let term = self.terms.get_index(*index);
                    let mut rewrites = HashSet::new();

                    for sub_index in 0..term.len() {
                        match term[sub_index] {
                            TermIR::Term(num) => {
                                if num == *pre_index {
                                    rewrites.insert(sub_index);
                                }
                            }
                            _ => ()
                        }
                    }

                    if !rewrites.is_empty() {
                        let internal = true;

                        let mut new_term = term.clone();

                        for sub_index in rewrites {
                            let elem = new_term.get_mut(sub_index).unwrap();
                            *elem = TermIR::Term(*post_index);
                        }
                        let new_index = self.terms.insert(new_term.to_vec().into_boxed_slice());
                        self.term_indices.insert(new_index);
                    }
                }

                if top_level {
                    self.term_indices.insert(new_top_term_index);
                }
            }

            if !( external | internal ) {
                return (false, step)
            }

        }
        ( true, steps )
    }

    //fn translate( &self ) -> 

    //pub fn display

}