use std::usize;

use crate::de_bruijn::{DTerm};
use std::rc::Rc;

fn _substitute<'a>(body: Rc<DTerm<'a>>, with: Rc<DTerm<'a>>, depth: usize) -> Rc<DTerm<'a>> {
    match &*body {
        DTerm::DBVar(_name, index) => if *index == depth { with } else { body },
        DTerm::DUVar(_name) => body,
        DTerm::DAbs(abs_body) => Rc::new(DTerm::DAbs(_substitute(Rc::clone(abs_body), with, depth + 1))),
        DTerm::DApp(l, r) =>
            Rc::new(DTerm::DApp(_substitute(Rc::clone(l), Rc::clone(&with), depth), _substitute(Rc::clone(r), Rc::clone(&with), depth)))
    }
}

fn substitute<'a>(body: Rc<DTerm<'a>>, with: Rc<DTerm<'a>>) -> Rc<DTerm<'a>> {
    _substitute(body, with, 0)
}

pub fn beta_reduction(dterm: Rc<DTerm>) -> Rc<DTerm> {
    match &*dterm {
        DTerm::DApp(l, r) => match &**l {
            DTerm::DAbs(body) => substitute(Rc::clone(body), Rc::clone(r)),
            _ => Rc::new(DTerm::DApp(beta_reduction(Rc::clone(l)), beta_reduction(Rc::clone(r))))
        },
        DTerm::DAbs(body) => Rc::new(DTerm::DAbs(beta_reduction(Rc::clone(body)))),
        DTerm::DBVar(name, index) => Rc::new(DTerm::DBVar(name, *index)),
        DTerm::DUVar(name) => Rc::new(DTerm::DUVar(name))
    }
}

fn contains(dterm: Rc<DTerm>, depth: usize) -> bool {
    match &*dterm {
        DTerm::DAbs(body) => contains(Rc::clone(body), depth + 1),
        DTerm::DApp(l, r) => contains(Rc::clone(l), depth) || contains(Rc::clone(r), depth),
        DTerm::DUVar(_name) => false,
        DTerm::DBVar(_name, index) => *index == depth
    }
}

pub fn eta_reduction(dterm: Rc<DTerm>) -> Rc<DTerm> {
    match &*dterm {
        DTerm::DApp(l, r) => Rc::new(DTerm::DApp(eta_reduction(Rc::clone(l)), eta_reduction(Rc::clone(r)))),
        DTerm::DAbs(body) => match &**body {
            DTerm::DApp(l, r) => match &**r {
                DTerm::DBVar(_name, index) => {
                    if *index == 0 && !contains(Rc::clone(l), 0) {
                        Rc::clone(l)
                    } else {
                        Rc::new(DTerm::DAbs(Rc::clone(body)))
                    }
                },
                _ => Rc::new(DTerm::DAbs(Rc::clone(body)))
            },
            _ => Rc::new(DTerm::DAbs(Rc::clone(body)))
        },
        DTerm::DUVar(name) => Rc::new(DTerm::DUVar(name)),
        DTerm::DBVar(name, index) => Rc::new(DTerm::DBVar(name, *index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::de_bruijn::de_bruijn_index;

    #[test]
    fn it_beta_reduces() {
        let (_input, term) = parse("((λx.x) (y z))").unwrap();
        let dterm = de_bruijn_index(term);
        let reduced = beta_reduction(Rc::new(dterm));
        let expected = DTerm::DApp(Rc::new(DTerm::DUVar("y")), Rc::new(DTerm::DUVar("z")));
        assert_eq!(expected, *reduced);
    }

    #[test]
    fn it_eta_reduces() {
        let (_input, term) = parse("(λx.(f x))").unwrap();
        let dterm = de_bruijn_index(term);
        let reduced = eta_reduction(Rc::new(dterm));
        let expected = DTerm::DUVar("f");
        assert_eq!(expected, *reduced);
    }

    #[test]
    fn it_does_not_eta_reduce_identity() {
        let (_input, term) = parse("(λx.x)").unwrap();
        let dterm = de_bruijn_index(term);
        let reduced = eta_reduction(Rc::new(dterm));
        let expected = DTerm::DAbs(Rc::new(DTerm::DBVar("x", 0)));
        assert_eq!(expected, *reduced);
    }
}
