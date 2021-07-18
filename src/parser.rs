extern crate nom;
use nom::{
    IResult,
    branch::alt,
    combinator::map_res,
    sequence::{delimited, tuple},
    character::complete::{alpha1, char}
};

#[derive(Debug,PartialEq)]
pub enum Term<'a> {
    Var(&'a str),
    Abs(Box<Term<'a>>, Box<Term<'a>>),
    App(Box<Term<'a>>, Box<Term<'a>>)
}

fn from_variable(input: &str) -> Result<Term, &str> {
    Ok(Term::Var(input))
}

fn variable(input: &str) -> IResult<&str, Term> {
    map_res(alpha1, from_variable)(input)
}

fn from_abstraction<'a>(input: (char, Term<'a>, char, Term<'a>)) -> Result<Term<'a>, &'a str> {
    let (_, v, _, t) = input;
    Ok(Term::Abs(Box::new(v), Box::new(t)))
}

fn abstraction(input: &str) -> IResult<&str, Term> {
    map_res(
        delimited(char('('), tuple((char('λ'), variable, char('.'), term)), char(')')),
        from_abstraction
    )(input)
}

fn from_application<'a>(input: (Term<'a>, char, Term<'a>)) -> Result<Term<'a>, &'a str> {
    let (l, _, r) = input;
    Ok(Term::App(Box::new(l), Box::new(r)))
}

fn application(input: &str) -> IResult<&str, Term> {
    map_res(
        delimited(char('('), tuple((term, char(' '), term)), char(')')),
        from_application
    )(input)
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((application, abstraction, variable))(input)
}

pub fn parse(input: &str) -> IResult<&str, Term> {
  term(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_abstraction() {
        let (input, result) = parse("(λx.x)").unwrap();
        assert_eq!(input, "");
        let var_x1 = Box::new(Term::Var("x"));
        let var_x2 = Box::new(Term::Var("x"));
        assert_eq!(result, Term::Abs(var_x1, var_x2));
    }

    #[test]
    fn it_parses_application() {
        let (input, result) = parse("(x y)").unwrap();
        assert_eq!(input, "");
        let var_x = Box::new(Term::Var("x"));
        let var_y = Box::new(Term::Var("y"));
        assert_eq!(result, Term::App(var_x, var_y));
    }

    #[test]
    fn it_parses_variables() {
        let (input, result) = parse("x").unwrap();
        assert_eq!(input, "");
        assert_eq!(result, Term::Var("x"));
    }

    #[test]
    fn it_parses_variables_abstraction_application() {
        let (input, result) = parse("((λx.x) y)").unwrap();
        assert_eq!(input, "");
        let var_x1 = Box::new(Term::Var("x"));
        let var_x2 = Box::new(Term::Var("x"));
        let var_y = Box::new(Term::Var("y"));
        assert_eq!(result, Term::App(Box::new(Term::Abs(var_x1, var_x2)), var_y));
    }

    #[test]
    fn it_parses_left_application() {
      let (input, result) = parse("((x y) z)").unwrap();
      assert_eq!(input, "");
      let var_x = Box::new(Term::Var("x"));
      let var_y = Box::new(Term::Var("y"));
      let var_z = Box::new(Term::Var("z"));
      assert_eq!(result, Term::App(Box::new(Term::App(var_x, var_y)), var_z));
    }

    #[test]
    fn it_parses_right_application() {
      let (input, result) = parse("(x (y z))").unwrap();
      assert_eq!(input, "");
      let var_x = Box::new(Term::Var("x"));
      let var_y = Box::new(Term::Var("y"));
      let var_z = Box::new(Term::Var("z"));
      assert_eq!(result, Term::App(var_x, Box::new(Term::App(var_y, var_z))));
    }
}
