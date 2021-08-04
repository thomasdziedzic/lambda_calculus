#![allow(nonstandard_style)]
// Generated from LambdaCalculus.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::lambdacalculusparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link LambdaCalculusParser}.
 */
pub trait LambdaCalculusVisitor<'input>: ParseTreeVisitor<'input,LambdaCalculusParserContextType>{
	/**
	 * Visit a parse tree produced by the {@code parens}
	 * labeled alternative in {@link LambdaCalculusParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_parens(&mut self, ctx: &ParensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code application}
	 * labeled alternative in {@link LambdaCalculusParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_application(&mut self, ctx: &ApplicationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code abstraction}
	 * labeled alternative in {@link LambdaCalculusParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_abstraction(&mut self, ctx: &AbstractionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code variable}
	 * labeled alternative in {@link LambdaCalculusParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code let}
	 * labeled alternative in {@link LambdaCalculusParser#term}.
	 * @param ctx the parse tree
	 */
	fn visit_let(&mut self, ctx: &LetContext<'input>) { self.visit_children(ctx) }


}
