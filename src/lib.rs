use std::io;
use std::rc::Rc;

use de_bruijn::de_bruijn_index;
use parser::parse;
use reducer::{beta_reduction, eta_reduction};

pub mod parser;
pub mod de_bruijn;
pub mod reducer;

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input
}

fn eval(input: String) -> String {
    let (_input, term) = parse(&input[..]).unwrap();

    let mut previous = Rc::new(de_bruijn_index(term));

    let mut current = eta_reduction(beta_reduction(Rc::clone(&previous)));

    while *previous != *current {
        previous = Rc::clone(&current);
        current = eta_reduction(beta_reduction(current));
    }

    format!("{}", current)
}

pub fn read_eval_print() {
    let input = read();
    let output = eval(input);
    println!("{}", output);
}
