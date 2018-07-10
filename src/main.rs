mod formula;
mod dimacs;
mod solver;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let formula = dimacs::reader(&args[1]);
    let answer = solver::solver(formula);
    match answer {
     true => println!("SAT"),
     false=> println!("UNSAT")
     }
}
