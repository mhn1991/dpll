pub use formula::clause::literal::Literal;
pub mod literal;

#[derive(Debug,Clone)]
pub struct Clause{
    pub list:Vec<Literal>,
}


impl Clause{
     pub fn new()->Clause{
     	 Clause{
		list:vec!(),
	}
     }
     pub fn add_lit(mut clause:Clause,lit:Literal)-> Clause{
     	 clause.list.push(lit);
	 return clause;
     }

     pub fn remove_lit_clause(clause:&mut Clause,literal:Literal){
     	 clause.list.retain(|&x| x != literal)
     }
}