#![allow(nonstandard_style)]
// Generated from LambdaCalculus.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::lambdacalculusparser::*;

pub trait LambdaCalculusListener<'input> : ParseTreeListener<'input,LambdaCalculusParserContextType>{

/**
 * Enter a parse tree produced by the {@code parens}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn enter_parens(&mut self, _ctx: &ParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parens}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn exit_parens(&mut self, _ctx: &ParensContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code application}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn enter_application(&mut self, _ctx: &ApplicationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code application}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn exit_application(&mut self, _ctx: &ApplicationContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code abstraction}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn enter_abstraction(&mut self, _ctx: &AbstractionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code abstraction}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn exit_abstraction(&mut self, _ctx: &AbstractionContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code variable}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code variable}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }

/**
 * Enter a parse tree produced by the {@code let}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn enter_let(&mut self, _ctx: &LetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code let}
 * labeled alternative in {@link LambdaCalculusParser#term}.
 * @param ctx the parse tree
 */
fn exit_let(&mut self, _ctx: &LetContext<'input>) { }

}
