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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evaluates_a_term() {
        let output = eval(String::from("((λx.x) y)"));
        assert_eq!(output, String::from("y(unbound)"));
    }

    #[test]
    fn it_evaluates_the_omega_combinator() {
        let output = eval(String::from("((λx.(x x)) y)"));
        assert_eq!(output, String::from("(y(unbound) y(unbound))"));
    }

    #[test]
    fn it_evaluates_the_big_omega_combinator() {
        let output = eval(String::from("((λx.(x x)) (λx.(x x)))"));
        assert_eq!(output, String::from("((λ.(x(0) x(0))) (λ.(x(0) x(0))))"));
    }

    #[test]
    fn it_evaluates_the_c_combinator() {
        let output = eval(String::from("((((λx.(λy.(λz.(x (y z))))) f) g) h)"));
        assert_eq!(output, String::from("(f(unbound) (g(unbound) h(unbound)))"));
    }
}
