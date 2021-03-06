#[derive(Debug,Copy, Clone)]
pub struct Literal{
    id: i32,
    value: Option<bool>,
}

impl Literal{
     pub fn new(id:i32)-> Literal{
     	 Literal{
		id:id,
		value:None,
	}
     }
    pub fn negate(literal:Literal)-> Literal{
     	 let mut literal = literal;
	 literal.id = literal.id * -1;
	 return literal;
     }
}

impl PartialEq for Literal{
     fn eq(&self,right:&Literal) -> bool{
     	self.id == right.id
     }
}