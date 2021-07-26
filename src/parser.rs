extern crate nom;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0},
    combinator::map_res,
    error::ParseError,
    sequence::{delimited, tuple}
};

#[derive(Debug,PartialEq)]
pub enum AST<'a> {
    Var(&'a str),
    Abs(Box<AST<'a>>, Box<AST<'a>>),
    App(Box<AST<'a>>, Box<AST<'a>>),
    Let(&'a str, Box<AST<'a>>, Box<AST<'a>>),
}

fn from_variable(input: &str) -> Result<AST, &str> {
    Ok(AST::Var(input))
}

fn variable(input: &str) -> IResult<&str, AST> {
    map_res(alpha1, from_variable)(input)
}

fn from_abstraction<'a>(input: (char, AST<'a>, char, AST<'a>)) -> Result<AST<'a>, &'a str> {
    let (_, v, _, t) = input;
    Ok(AST::Abs(Box::new(v), Box::new(t)))
}

fn abstraction(input: &str) -> IResult<&str, AST> {
    map_res(
        delimited(char('('), tuple((char('λ'), variable, char('.'), ast)), char(')')),
        from_abstraction
    )(input)
}

fn from_application<'a>(input: (AST<'a>, char, AST<'a>)) -> Result<AST<'a>, &'a str> {
    let (l, _, r) = input;
    Ok(AST::App(Box::new(l), Box::new(r)))
}

fn application(input: &str) -> IResult<&str, AST> {
    map_res(
        delimited(char('('), tuple((ast, char(' '), ast)), char(')')),
        from_application
    )(input)
}

fn from_let<'a>(input: (&'a str, &'a str, char, AST<'a>, &'a str, AST<'a>)) -> Result<AST<'a>, &'a str> {
    let (_, var, _, val, _, body) = input;
    Ok(AST::Let(var, Box::new(val), Box::new(body)))
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

fn let_term(input: &str) -> IResult<&str, AST> {
    map_res(tuple((ws(tag("let")), ws(alpha1), ws(char('=')), ws(ast), ws(tag("in")), ws(ast))), from_let)(input)
}

fn ast(input: &str) -> IResult<&str, AST> {
    alt((let_term, application, abstraction, variable))(input)
}

pub fn parse(input: &str) -> IResult<&str, AST> {
  ast(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_abstraction() {
        let (input, result) = parse("(λx.x)").unwrap();
        assert_eq!(input, "");
        let var_x1 = Box::new(AST::Var("x"));
        let var_x2 = Box::new(AST::Var("x"));
        assert_eq!(result, AST::Abs(var_x1, var_x2));
    }

    #[test]
    fn it_parses_application() {
        let (input, result) = parse("(x y)").unwrap();
        assert_eq!(input, "");
        let var_x = Box::new(AST::Var("x"));
        let var_y = Box::new(AST::Var("y"));
        assert_eq!(result, AST::App(var_x, var_y));
    }

    #[test]
    fn it_parses_variables() {
        let (input, result) = parse("x").unwrap();
        assert_eq!(input, "");
        assert_eq!(result, AST::Var("x"));
    }

    #[test]
    fn it_parses_variables_abstraction_application() {
        let (input, result) = parse("((λx.x) y)").unwrap();
        assert_eq!(input, "");
        let var_x1 = Box::new(AST::Var("x"));
        let var_x2 = Box::new(AST::Var("x"));
        let var_y = Box::new(AST::Var("y"));
        assert_eq!(result, AST::App(Box::new(AST::Abs(var_x1, var_x2)), var_y));
    }

    #[test]
    fn it_parses_left_application() {
      let (input, result) = parse("((x y) z)").unwrap();
      assert_eq!(input, "");
      let var_x = Box::new(AST::Var("x"));
      let var_y = Box::new(AST::Var("y"));
      let var_z = Box::new(AST::Var("z"));
      assert_eq!(result, AST::App(Box::new(AST::App(var_x, var_y)), var_z));
    }

    #[test]
    fn it_parses_right_application() {
      let (input, result) = parse("(x (y z))").unwrap();
      assert_eq!(input, "");
      let var_x = Box::new(AST::Var("x"));
      let var_y = Box::new(AST::Var("y"));
      let var_z = Box::new(AST::Var("z"));
      assert_eq!(result, AST::App(var_x, Box::new(AST::App(var_y, var_z))));
    }

    #[test]
    fn it_parses_let() {
        let (input, result) = parse("let I = (λx.x) in (I a)").unwrap();
        assert_eq!(input, "");
        let var_x1 = Box::new(AST::Var("x"));
        let var_x2 = Box::new(AST::Var("x"));
        let var_i = Box::new(AST::Var("I"));
        let var_a= Box::new(AST::Var("a"));
        assert_eq!(result, AST::Let("I", Box::new(AST::Abs(var_x1, var_x2)), Box::new(AST::App(var_i, var_a))));
    }
}
