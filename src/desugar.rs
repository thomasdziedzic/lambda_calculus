use crate::parser::AST;

#[derive(Debug,PartialEq)]
pub enum Term<'a> {
    Var(&'a str),
    Abs(Box<Term<'a>>, Box<Term<'a>>),
    App(Box<Term<'a>>, Box<Term<'a>>),
}

pub fn desugar(ast: AST) -> Term {
  match ast {
    AST::Abs(l, r) => Term::Abs(Box::new(desugar(*l)), Box::new(desugar(*r))),
    AST::App(l, r) => Term::App(Box::new(desugar(*l)), Box::new(desugar(*r))),
    AST::Var(var) => Term::Var(var),
    AST::Let(var,val,term) => Term::App(Box::new(Term::Abs(Box::new(Term::Var(var)), Box::new(desugar(*term)))), Box::new(desugar(*val))),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_desugars_let() {
    let ast = AST::Let("I", Box::new(AST::Abs(Box::new(AST::Var("x")), Box::new(AST::Var("x")))), Box::new(AST::Var("I")));
    let var_i1 = Box::new(Term::Var("I"));
    let var_i2 = Box::new(Term::Var("I"));
    let var_x1 = Box::new(Term::Var("x"));
    let var_x2 = Box::new(Term::Var("x"));
    let expected = Term::App(Box::new(Term::Abs(var_i1, var_i2)), Box::new(Term::Abs(var_x1, var_x2)));
    assert_eq!(desugar(ast), expected);
  }
}
