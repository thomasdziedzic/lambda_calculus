use std::io;
use std::rc::Rc;

use de_bruijn::de_bruijn_index;
use parser::parse;
use reducer::{beta_reduction, eta_reduction};
use desugar::desugar;

pub mod parser;
pub mod desugar;
pub mod de_bruijn;
pub mod reducer;

fn read() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input
}

fn eval(input: String) -> String {
    let (_input, term) = parse(&input[..]).unwrap();
    let term = desugar(term);

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
    fn it_handles_variables() {
        let output = eval(String::from("x"));
        assert_eq!(output, String::from("x(unbound)"));
    }

    #[test]
    fn it_handles_abstraction() {
        let output = eval(String::from("(λx.x)"));
        assert_eq!(output, String::from("(λ.x(0))"));
    }

    #[test]
    fn it_handles_application() {
        let output = eval(String::from("(x y)"));
        assert_eq!(output, String::from("(x(unbound) y(unbound))"));
    }

    #[test]
    fn it_evaluates_the_identity_combinator() {
        let output = eval(String::from("((λx.x) y)"));
        assert_eq!(output, String::from("y(unbound)"));
    }

    #[test]
    fn it_evaluates_the_k_combinator() {
        let output = eval(String::from("(((λx.(λy.x)) a) b)"));
        assert_eq!(output, String::from("a(unbound)"));
    }

    #[test]
    fn it_evaluates_the_s_combinator() {
        let output = eval(String::from("((((λx.(λy.(λz.((x z) (y z))))) a) b) c)"));
        assert_eq!(output, String::from("((a(unbound) c(unbound)) (b(unbound) c(unbound)))"));
    }

    #[test]
    fn it_evaluates_the_b_combinator() {
        let output = eval(String::from("((((λx.(λy.(λz.(x (y z))))) f) g) h)"));
        assert_eq!(output, String::from("(f(unbound) (g(unbound) h(unbound)))"));
    }

    #[test]
    fn it_evaluates_the_c_combinator() {
        let output = eval(String::from("((((λx.(λy.(λz.((x z) y)))) f) g) h)"));
        assert_eq!(output, String::from("((f(unbound) h(unbound)) g(unbound))"));
    }

    #[test]
    fn it_evaluates_the_w_combinator() {
        let output = eval(String::from("(((λx.(λy.((x y) y))) a) b)"));
        assert_eq!(output, String::from("((a(unbound) b(unbound)) b(unbound))"));
    }

    // The u combinator is the same thing as the omega combinator so no need to test it.
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
    fn it_can_evaluate_a_y_combinator_with_a_terminating_function() {
        let output = eval(String::from("((λg.((λx.(g (x x))) (λx.(g (x x))))) (λx.(λy.y)))"));
        assert_eq!(output, String::from("(λ.y(0))"));
    }

    #[test]
    fn it_handles_booleans() {
        let output = eval(String::from("
            let TRUE = (λx.(λy.x)) in
            let FALSE = (λx.(λy.y)) in
            let AND = (λp.(λq.((p q) p))) in
            ((AND TRUE) FALSE)
            "));
        assert_eq!(output, String::from("(λ.(λ.y(0)))"));
    }

    #[test]
    fn it_handles_church_numerals() {
        let output = eval(String::from("
            let zero = (λf.(λx.x)) in
            let succ = (λn.(λf.(λx.(f ((n f) x))))) in
            let one = (succ zero) in
            let two = (succ one) in
            let three = (succ two) in
            let plus = (λm.(λn.(λf.(λx.((m f) ((n f) x)))))) in
            ((plus two) three)
            "));
        assert_eq!(output, String::from("(λ.(λ.(f(1) (f(1) (f(1) (f(1) (f(1) x(0))))))))"));
    }

    #[test]
    fn it_handles_pairs() {
        let output = eval(String::from("
            let pair = (λx.(λy.(λf.((f x) y)))) in
            let first = (λp.(p (λx.(λy.x)))) in
            let second = (λp.(p (λx.(λy.y)))) in
            let zero = (λf.(λx.x)) in
            let succ = (λn.(λf.(λx.(f ((n f) x))))) in
            let one = (succ zero) in
            let nums = ((pair zero) one) in
            let flip = (λp.((pair (second p)) (first p))) in
            (flip nums)
            "));
        assert_eq!(output, String::from("(λ.((f(0) (λ.(λ.(f(1) x(0))))) (λ.(λ.x(0)))))"));
    }
}
