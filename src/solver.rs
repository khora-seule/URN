use crate::parser::Term;
use std::collections::HashSet;
use indexmap::IndexSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TermIR {
    Name(usize),
    Term(usize),
}

#[derive(PartialEq, Eq, Clone)]
enum TreeIR {
    Leaf(usize),
    Tree(Vec<TreeIR>),
}

impl TreeIR {
    fn replace( self, rule: &(TreeIR, TreeIR) ) -> TreeIR {
        if self == rule.0 {
            return rule.1.clone()
        }
        match self {
            TreeIR::Tree(ptr) => {
                return TreeIR::Tree((*ptr).iter().map( |x| x.clone().replace( rule ) ).collect::<Vec<_>>())
            }
            TreeIR::Leaf(_) => {
                return self
            }
        }
    }
}

#[derive(Debug)]
pub enum OutputTree {
    Leaf(Box<str>),
    Tree(Vec<OutputTree>),
}

impl fmt::Display for OutputTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputTree::Leaf(name) => {
                write!(f, " {} ", name)
            }
            OutputTree::Tree(tree) => {
                write!(f, "{{").unwrap();
                for sub_tree in tree.iter().rev() {
                    write!(f, "{}", sub_tree).unwrap();
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
struct TermSet(IndexSet<Box<[TermIR]>>);

impl TermSet {

    fn insert_raw( &mut self, names: &Box<[Box<str>]>, parsed_term: &[Term] ) -> usize {

        let mut new_term = Vec::new();

        for sub_term in parsed_term.into_iter() {
            match sub_term {
                Term::Word(name) => new_term.push(TermIR::Name(names.iter().position(|x| x == name).unwrap() )),
                Term::Sentence(ptr,_) => new_term.push(TermIR::Term(self.insert_raw( names, &*ptr ))),
                _ => todo!(),
            }
        }
        self.insert(new_term.into_boxed_slice())
    }
    fn insert( &mut self, term: Box<[TermIR]> ) -> usize {
        let (index,_) = self.0.insert_full(term);
        index
    }
    fn get_index( &self, index: &usize ) -> &Box<[TermIR]> {
        self.0.get_index( *index ).unwrap()
    }
    fn deep_get_index( &self, index: &usize ) -> TreeIR {

        let mut term = Vec::new();

        for sub_term in self.get_index( index ).clone().into_iter() {
            match sub_term {
                TermIR::Name(name_index) => term.push(TreeIR::Leaf(*name_index)),
                TermIR::Term(sub_index) => term.push(self.deep_get_index(sub_index)),
            }
        }
        TreeIR::Tree(term)
    }
}


#[derive(Debug)]
pub struct TermTable {
    names: Box<[Box<str>]>,
    terms: TermSet,
    term_indices: HashSet<usize>,
    rule_indices: Box<[(usize,usize)]>
}
impl TermTable {
    pub fn build_term_table( parsed_terms: Box<[Term]>, parsed_rules: Box<[Term]>, names:  Box<[Box<str>]> ) -> TermTable  {
        
        let mut terms = TermSet(IndexSet::new());
        let mut term_indices = HashSet::new();
        let mut rule_indices = IndexSet::new();

        for rule_pair in parsed_rules.chunks(2) {
            let pre_index;
            let post_index;
            match &rule_pair[0] {
                Term::Sentence(ptr,_) => {

                    pre_index = terms.insert_raw(&names, &*ptr);
                }
                _ => todo!(),
            }
            match &rule_pair[1] {
                Term::Sentence(ptr,_) => {
                    post_index = terms.insert_raw(&names, &*ptr);
                }
                _ => todo!(),
            }
            rule_indices.insert((pre_index,post_index));
        }

        for term in parsed_terms.into_iter() {
            let index;
            match term {
                Term::Sentence(ptr,_) => {
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

    fn insert( &mut self, term: &Vec<TreeIR> ) -> usize {

        let mut new_term = Vec::new();

        for sub_tree in term.into_iter() {
            match sub_tree {
                TreeIR::Leaf(node) => new_term.push(TermIR::Name(*node)),
                TreeIR::Tree(vec) => {
                    let sub_index = self.insert( &vec );
                    new_term.push(TermIR::Term(sub_index));
                }
            }
        }
        let index = self.terms.insert(new_term.into_boxed_slice());
        index
    }

    fn insert_update( &mut self, term: &Vec<TreeIR> ) -> usize {
        let index = self.insert( term );
        self.term_indices.insert(index);
        index
    }

    fn top_contains( &self, index: &usize ) -> bool {
        self.term_indices.contains( &index )
    }

    fn sub_contains ( &self, index: &usize, term: &TermIR ) -> bool {

        let mut visitable = HashSet::from([index]);
        let mut visited = HashSet::new();

        while !visitable.difference(&visited).collect::<HashSet<_>>().is_empty() {

            let itinerary = visitable.drain().collect::<Vec<_>>().clone();

            for term_index in itinerary {
                visited.insert(term_index);

                for sub_term in self.terms.get_index(&term_index).iter() {
                    match sub_term {
                        TermIR::Term(sub_index) => {
                            if sub_term == term {
                                return true
                            }
                            visitable.insert(sub_index);
                        }
                        TermIR::Name(_) => {
                            if sub_term == term {
                                return true
                            }
                        }
                    }
                }
            }
        }
        // This is false since any value that would cause the function to evaluate to true
        // Should also cause the function to short-circuit early
        false
    }
    fn internal_contains( &self, index: &usize ) -> bool {
        self.term_indices.clone()
            .iter()
            .any( |x| self.sub_contains( x, &TermIR::Term( *index ) ) )
    }

    pub fn full_contains ( &self, index: &usize ) -> ( bool, bool ) {
        ( self.top_contains( index ), self.internal_contains( index ) )
    }

    //  Rewrites exactly one step
    //  Returns a bool indicating whether any rewrite occured
 
    pub fn rewrite( &mut self ) -> bool {

        for index_pair in self.rule_indices.clone().iter() {
            let (top, internal) = self.full_contains( &index_pair.0 );
            let any = top | internal;
            if any {
                let rule = &(self.terms.deep_get_index( &index_pair.0), self.terms.deep_get_index( &index_pair.1));
                if internal {
                    let targets = self.term_indices
                        .clone()
                        .iter()
                        .filter( |x| self.sub_contains( *x, &TermIR::Term(index_pair.0)))
                        .map( |x| *x )
                        .collect::<HashSet<usize>>();

                    self.term_indices = self.term_indices
                        .clone()
                        .difference(&targets)
                        .map( |x| *x )
                        .collect::<HashSet<_>>();
                        
                    let trees = targets
                        .iter()
                        .map( |x| self.terms
                            .deep_get_index( x )
                            .replace( rule ) )
                        .collect::<Vec<_>>();
                    for tree in trees {
                        match tree {
                            TreeIR::Tree(tree) => {
                                self.insert_update(&tree);
                            }
                            TreeIR::Leaf(_) => (),
                        }
                    }
                }
                if top {
                    self.term_indices.take(&index_pair.0);
                    self.term_indices.insert(index_pair.1);
                }
                return true
            }
        }
        false
    }

    fn translate( &self, tree: &TreeIR ) -> OutputTree {

        let mut translation = Vec::new();

        match tree {
            TreeIR::Leaf(name_index) => {
                return OutputTree::Leaf(self.names[*name_index])
            }
            TreeIR::Tree(sub_tree) => {
                for sub_sub_tree in sub_tree.iter() {
                    translation.push( self.translate( sub_sub_tree ) )
                }
            }
        }
        OutputTree::Tree(translation)
    }

    fn get( &self, index: &usize ) -> OutputTree {

        let tree = &self.terms.deep_get_index(index);

        self.translate( tree )
    }

    pub fn display( &self ) -> Vec<OutputTree> {
        self.term_indices
            .iter()
            .map( |x| self.get(x) )
            .collect::<Vec<_>>()
    }

}