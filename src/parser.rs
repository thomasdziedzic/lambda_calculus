use antlr_rust::rule_context::CustomRuleContext;
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
    #[allow(dead_code)]
    _inputs: Vec<&'i str>,
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

    terms.ast
}

impl<'i> ParseTreeVisitor<'i, LambdaCalculusParserContextType> for MyLambdaCalculusParser<'i> {}

impl<'i> LambdaCalculusVisitor<'i> for MyLambdaCalculusParser<'i> {
    /*
    fn visit_term(&mut self, ctx: &crate::TermContext<'i>) {
        let left = ctx.
        println!("{}", ctx.get_rule_index());
    }
    */
    /*
    fn visit_parens(&mut self, ctx: &crate::ParensContext<'i>) {
        let m_term = ctx.term();
        match m_term {
            None => None,
            Some(term) => term.
        }
    }
    */
}

/*
pub fn parse(input: &str) -> IResult<&str, AST> {
  ast(input)
}
*/

pub fn parse(input: &str) -> Result<AST, &str> {
    Ok(AST::Var("x"))
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_abstraction() {
        let (input, result) = parse("(λx.x)").unwrap();
        assert_eq!(input, "");
        let var_x = Box::new(AST::Var("x"));
        assert_eq!(result, AST::Abs(vec!["x"], var_x));
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
        let var_x = Box::new(AST::Var("x"));
        let var_y = Box::new(AST::Var("y"));
        assert_eq!(result, AST::App(Box::new(AST::Abs(vec!["x"], var_x)), var_y));
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
        let var_x = Box::new(AST::Var("x"));
        let var_i = Box::new(AST::Var("I"));
        let var_a= Box::new(AST::Var("a"));
        assert_eq!(result, AST::Let("I", Box::new(AST::Abs(vec!["x"], var_x)), Box::new(AST::App(var_i, var_a))));
    }

    #[test] #[ignore]
    fn it_associates_abstractions_to_the_right() {
        let (input, result) = parse("λx.λy.x").unwrap();
        assert_eq!(input, "");
        assert_eq!(result, AST::Abs(vec!["x"], Box::new(AST::Abs(vec!["y"], Box::new(AST::Var("x"))))))
    }

    #[test] #[ignore]
    fn it_associates_applications_to_the_left() {
        let (input, result) = parse("a b c").unwrap();
        assert_eq!(input, "");
        let a = Box::new(AST::Var("a"));
        let b = Box::new(AST::Var("b"));
        let c = Box::new(AST::Var("c"));
        assert_eq!(result, AST::App(Box::new(AST::App(a, b)), c));
    }
}
*/
