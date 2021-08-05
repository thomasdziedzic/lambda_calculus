use std::borrow::{Cow};

use antlr_rust::{InputStream, common_token_stream::CommonTokenStream, tree::ParseTreeVisitor};
use antlr_rust::tree::Visitable;
use crate::{LambdaCalculusLexer, LambdaCalculusParser, LambdaCalculusParserContextType, LambdaCalculusVisitor};

use crate::lambdacalculusparser::*;

#[derive(Debug,PartialEq)]
pub enum AST<'a> {
    Var(&'a str),
    Abs(Vec<&'a str>, Box<AST<'a>>),
    App(Box<AST<'a>>, Box<AST<'a>>),
    Let(&'a str, Box<AST<'a>>, Box<AST<'a>>),
}

pub struct MyLambdaCalculusParser<'i> {
    _inputs: Vec<AST<'i>>
}

fn str_to_ast<'input>(input: &str) -> Option<AST> {
    let lexer = LambdaCalculusLexer::new(InputStream::new(input.into()));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LambdaCalculusParser::new(token_source);
    let result = parser.term().expect("failed to parse");

    let mut terms = MyLambdaCalculusParser {
        _inputs: vec![]
    };

    result.accept(&mut terms);

    terms._inputs.pop()
}

impl<'i> ParseTreeVisitor<'i, LambdaCalculusParserContextType> for MyLambdaCalculusParser<'i> {}

impl<'i> LambdaCalculusVisitor<'i> for MyLambdaCalculusParser<'i> {
    fn visit_parens(&mut self, ctx: &ParensContext<'i>) {
        self.visit_children(ctx);
    }

    fn visit_application(&mut self, ctx: &ApplicationContext<'i>) {
        ctx.left.as_ref().unwrap().accept(self);
        ctx.right.as_ref().unwrap().accept(self);

        let right_val = self._inputs.pop().unwrap();
        let left_val = self._inputs.pop().unwrap();

        let app = AST::App(Box::new(left_val), Box::new(right_val));
        self._inputs.push(app);
    }

    fn visit_abstraction(&mut self, ctx: &AbstractionContext<'i>) {
        ctx.abs_body.as_ref().unwrap().accept(self);

        let mut vars = vec![];

        // while implementing this for loop, discovered a bug in the nightly rust compiler:
        // https://github.com/rust-lang/rust/issues/87657
        // workaround is to currently use nightly-2021-07-20 which doesn't have this bug
        for variable in &ctx.variables {
            if let Cow::Borrowed(var_str) = variable.text {
                vars.push(var_str);
            }
        }

        let body = self._inputs.pop().unwrap();
        let abs = AST::Abs(vars, Box::new(body));
        self._inputs.push(abs);
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'i>) {
        if let Cow::Borrowed(var_str) = ctx.variable.as_ref().unwrap().text {
            let var = AST::Var(var_str);
            self._inputs.push(var);
        }
    }

    fn visit_let(&mut self, ctx: &LetContext<'i>) {
        ctx.assignment.as_ref().unwrap().accept(self);
        ctx.body.as_ref().unwrap().accept(self);

        let body = self._inputs.pop().unwrap();
        let assignment = self._inputs.pop().unwrap();

        if let Cow::Borrowed(var_str) = ctx.variable.as_ref().unwrap().text {
            let let_statement = AST::Let(var_str, Box::new(assignment), Box::new(body));
            self._inputs.push(let_statement);
        }
    }
}

pub fn parse(input: &str) -> Result<AST, &str> {
    match str_to_ast(input) {
        None => Err("Did not get anything back from the parser"),
        Some(ast) => Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_abstraction() {
        let result = parse("(λx.x)").unwrap();
        let var_x = Box::new(AST::Var("x"));
        assert_eq!(result, AST::Abs(vec!["x"], var_x));
    }

    #[test]
    fn it_parses_application() {
        let result = parse("(x y)").unwrap();
        let var_x = Box::new(AST::Var("x"));
        let var_y = Box::new(AST::Var("y"));
        assert_eq!(result, AST::App(var_x, var_y));
    }

    #[test]
    fn it_parses_variables() {
        let result = parse("x").unwrap();
        assert_eq!(result, AST::Var("x"));
    }

    #[test]
    fn it_parses_variables_abstraction_application() {
        let result = parse("((λx.x) y)").unwrap();
        let var_x = Box::new(AST::Var("x"));
        let var_y = Box::new(AST::Var("y"));
        assert_eq!(result, AST::App(Box::new(AST::Abs(vec!["x"], var_x)), var_y));
    }

    #[test]
    fn it_parses_left_application() {
      let result = parse("((x y) z)").unwrap();
      let var_x = Box::new(AST::Var("x"));
      let var_y = Box::new(AST::Var("y"));
      let var_z = Box::new(AST::Var("z"));
      assert_eq!(result, AST::App(Box::new(AST::App(var_x, var_y)), var_z));
    }

    #[test]
    fn it_parses_right_application() {
      let result = parse("(x (y z))").unwrap();
      let var_x = Box::new(AST::Var("x"));
      let var_y = Box::new(AST::Var("y"));
      let var_z = Box::new(AST::Var("z"));
      assert_eq!(result, AST::App(var_x, Box::new(AST::App(var_y, var_z))));
    }

    #[test]
    fn it_parses_let() {
        let result = parse("let I = (λx.x) in (I a)").unwrap();
        let var_x = Box::new(AST::Var("x"));
        let var_i = Box::new(AST::Var("I"));
        let var_a= Box::new(AST::Var("a"));
        assert_eq!(result, AST::Let("I", Box::new(AST::Abs(vec!["x"], var_x)), Box::new(AST::App(var_i, var_a))));
    }

    #[test]
    fn it_associates_abstractions_to_the_right() {
        let result = parse("λx.λy.x").unwrap();
        assert_eq!(result, AST::Abs(vec!["x"], Box::new(AST::Abs(vec!["y"], Box::new(AST::Var("x"))))))
    }

    #[test]
    fn it_associates_applications_to_the_left() {
        let result = parse("a b c").unwrap();
        let a = Box::new(AST::Var("a"));
        let b = Box::new(AST::Var("b"));
        let c = Box::new(AST::Var("c"));
        assert_eq!(result, AST::App(Box::new(AST::App(a, b)), c));
    }

    #[test]
    fn it_associates_multiple_applications_within_multiple_abstractions() {
        let result = parse("λa.λb.a b").unwrap();
        let a = Box::new(AST::Var("a"));
        let b = Box::new(AST::Var("b"));
        let expected = AST::Abs(vec!["a"], Box::new(AST::Abs(vec!["b"], Box::new(AST::App(a, b)))));
        assert_eq!(result, expected);
    }

    #[test]
    fn it_ignores_comments() {
        let result = parse("
        (*
            multi
            line
            comment
        *)
        # single line comment
        x # comment goes after term
        # another single line comment
        (*
            another
            multi
            line
            comment
        *)
        ").unwrap();
        assert_eq!(result, AST::Var("x"));
    }

    #[test]
    fn it_ignores_single_line_comments() {
        let result = parse("
        # single line comment
        x
        # another single line comment
        ").unwrap();
        assert_eq!(result, AST::Var("x"));
    }
}
