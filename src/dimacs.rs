use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use formula::clause::literal::Literal;
use formula::clause::Clause;
use formula::Formula;

/// this funtion will read given file line by line 
pub fn reader(path: &str)-> Formula{
    let file = File::open(path).expect("file can not be opened");
    let buffer = BufReader::new(&file);
    let mut formula = Formula::new();
    for line in buffer.lines(){
    	match line {
	      Ok(l)=> {
	      	     let chars:Vec<&str> = l.split(' ').collect();
		     for char in chars{
		             match char.parse::<i32>() {
			                   Ok(int) =>{
					   	   formula =  Formula::add_clause(formula,chooser(&l));
					   },
						   Err(_) => continue,
					   }
					   }
	      },
	      _=> println!("can not read file!!"),
	}
    }
   return formula;
}

/// this function will take decision that line is comment or not
pub fn chooser(line:&str)-> Clause{
    let mut clause = Clause::new();
    let chars:Vec<&str> = line.split(' ').collect();
    for char in chars{
    	match char.parse::<i32>() {
	      Ok(int) => {
	      	    if int != 0
		    {
			clause = Clause::add_lit(clause,Literal::new(int));
		    }
	      },
	      Err(_) => continue,
	}
    }
    return clause;
}
