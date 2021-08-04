use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::{InputStream, common_token_stream::CommonTokenStream, tree::ParseTreeVisitor};
use antlr_rust::tree::Visitable;
use nom::bitvec::view::AsBits;
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
    #[allow(dead_code)]
    _inputs: Vec<AST<'i>>,
    pub(crate) ast: Option<AST<'i>>
}

fn str_to_ast<'input>(input: &str) -> Option<AST> {
    let lexer = LambdaCalculusLexer::new(InputStream::new(input.into()));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = LambdaCalculusParser::new(token_source);
    let result = parser.term().expect("failed to parse");

    let mut terms = MyLambdaCalculusParser {
        _inputs: vec![],
        ast: None
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

        let vars = vec![];
        for variable in ctx.variables {
            vars.push(variable.text.as_ref());
        }

        let body = self._inputs.pop().unwrap();
        let abs = AST::Abs(vars, Box::new(body));
        self._inputs.push(abs);
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'i>) {
        let var_str = ctx.variable.unwrap().text.as_ref();
        let var = AST::Var(var_str);
        self._inputs.push(var);
    }

    fn visit_let(&mut self, ctx: &LetContext<'i>) {
        ctx.assignment.as_ref().unwrap().accept(self);
        ctx.body.as_ref().unwrap().accept(self);

        let body = self._inputs.pop().unwrap();
        let assignment = self._inputs.pop().unwrap();
        let var_str = ctx.variable.unwrap().text.as_ref();
        let let_statement = AST::Let(var_str, Box::new(body), Box::new(assignment));
        self._inputs.push(let_statement);
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

    #[test] #[ignore]
    fn it_associates_abstractions_to_the_right() {
        let result = parse("λx.λy.x").unwrap();
        assert_eq!(result, AST::Abs(vec!["x"], Box::new(AST::Abs(vec!["y"], Box::new(AST::Var("x"))))))
    }

    #[test] #[ignore]
    fn it_associates_applications_to_the_left() {
        let result = parse("a b c").unwrap();
        let a = Box::new(AST::Var("a"));
        let b = Box::new(AST::Var("b"));
        let c = Box::new(AST::Var("c"));
        assert_eq!(result, AST::App(Box::new(AST::App(a, b)), c));
    }
}
