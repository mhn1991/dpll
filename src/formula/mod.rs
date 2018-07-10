pub use formula::clause::Clause;
pub mod clause;

#[derive(Debug)]
pub struct Formula{
    pub list: Vec<Clause>
}

impl Formula{
     pub fn new()->Formula{
     	 Formula{
		list :  Vec::new(),
	 }
     }

     pub fn add_clause(mut formula:Formula,clause:Clause)-> Formula{
     	 formula.list.push(clause);
	 return formula;
     }
}

impl Clone for Formula{
     fn clone(&self)->Formula{
     	Formula{
		list: self.list.clone()
	}
     }
}