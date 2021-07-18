use crate::parser::Term;
use std::{fmt, rc::Rc, vec::Vec};

/*
    This implements the De Bruijn Index with locally nameless variables, which is a form
    of De Bruijn indices where unbound variables keep their name.
    https://github.com/joshpadilla/tractatus/blob/master/doc/de-bruijn-indices.md#locally-nameless-variables
*/

#[derive(Debug,PartialEq)]
pub enum DTerm<'a> {
    DBVar(&'a str, usize),
    DUVar(&'a str),
    DAbs(Rc<DTerm<'a>>),
    DApp(Rc<DTerm<'a>>, Rc<DTerm<'a>>)
}

impl<'a> fmt::Display for DTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            DTerm::DBVar(name, index) => write!(f, "{}({})", name, index),
            DTerm::DUVar(name) => write!(f, "{}(unbound)", name),
            DTerm::DAbs(body) => {
                write!(f, "(λ.")?;
                body.fmt(f)?;
                write!(f, ")")
            },
            DTerm::DApp(l, r) => {
                write!(f, "(")?;
                l.fmt(f)?;
                write!(f, " ")?;
                r.fmt(f)?;
                write!(f, ")")
            }
        }
    }
}

struct Context<'a> {
    binders: Vec<&'a str>
}

fn _de_bruijn_index<'a>(term: Term<'a>, context: &mut Context<'a>) -> DTerm<'a> {
    match term {
        Term::Var(name) => {
            match context.binders.iter().rposition(|&x| x == name) {
                None => {
                    DTerm::DUVar(name)
                },
                Some(rpos) => {
                    // subtracting 1 to get a 0-indexed de bruijn index.
                    let index = context.binders.len() - rpos - 1;
                    DTerm::DBVar(name, index)
                }
            }
        },
        Term::Abs(binder, body) => {
            match *binder {
                Term::Var(bound_variable) => {
                    context.binders.push(bound_variable);
                },
                _ => panic!("Expected a bound variable in a lambda abstraction.")
            }

            let dbody = _de_bruijn_index(*body, context);
            context.binders.pop();
            DTerm::DAbs(Rc::new(dbody))
        },
        Term::App(left, right) => {
            let ldterm = _de_bruijn_index(*left, context);
            let rdterm = _de_bruijn_index(*right, context);
            DTerm::DApp(Rc::new(ldterm), Rc::new(rdterm))
        }
    }
}

pub fn de_bruijn_index(term: Term) -> DTerm {
    let mut context = Context {
        binders: Vec::new()
    };
    _de_bruijn_index(term, &mut context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn it_handles_unbound_variables() {
        let (_input, term) = parse("x").unwrap();
        let dterm = de_bruijn_index(term);
        let expected = DTerm::DUVar("x");
        assert_eq!(expected, dterm);
    }

    #[test]
    fn it_handles_abstraction_with_bound_variable() {
        let (_input, term) = parse("(λx.x)").unwrap();
        let dterm = de_bruijn_index(term);
        let expected = DTerm::DAbs(Rc::new(DTerm::DBVar("x", 0)));
        assert_eq!(expected, dterm);
    }

    #[test]
    fn it_handles_application() {
        let (_input, term) = parse("(x y)").unwrap();
        let dterm = de_bruijn_index(term);
        let unbound_x = DTerm::DUVar("x");
        let unbound_y = DTerm::DUVar("y");
        let expected = DTerm::DApp(Rc::new(unbound_x), Rc::new(unbound_y));
        assert_eq!(expected, dterm);
    }

    #[test]
    fn it_handles_all_forms_at_once() {
        let (_input, term) = parse("((λx.x) y)").unwrap();
        let dterm = de_bruijn_index(term);
        let expected_abs = DTerm::DAbs(Rc::new(DTerm::DBVar("x", 0)));
        let unbound_y = DTerm::DUVar("y");
        let expected = DTerm::DApp(Rc::new(expected_abs), Rc::new(unbound_y));
        assert_eq!(expected, dterm);
    }

    #[test]
    fn it_handles_multiple_bound_variables() {
        let (_input, term) = parse("(λx.(λy.x))").unwrap();
        let dterm = de_bruijn_index(term);
        let expected = DTerm::DAbs(Rc::new(DTerm::DAbs(Rc::new(DTerm::DBVar("x", 1)))));
        assert_eq!(expected, dterm);
    }
}
