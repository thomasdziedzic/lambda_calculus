// Generated from LambdaCalculus.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::lambdacalculuslistener::*;
use super::lambdacalculusvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const VARIABLE:isize=8; 
		pub const WS:isize=9;
	pub const RULE_term:usize = 0;
	pub const ruleNames: [&'static str; 1] =  [
		"term"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("'\u{03BB}'"), Some("'.'"), Some("'let'"), Some("'='"), Some("'in'"), 
		Some("'('"), Some("')'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;10]  = [
		None, None, None, None, None, None, None, None, Some("VARIABLE"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,LambdaCalculusParserExt, I, LambdaCalculusParserContextType , dyn LambdaCalculusListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LambdaCalculusTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LambdaCalculusParserContextType , dyn LambdaCalculusListener<'input> + 'a>;

/// Parser for LambdaCalculus grammar
pub struct LambdaCalculusParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> LambdaCalculusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				LambdaCalculusParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> LambdaCalculusParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LambdaCalculusParser<'input, I, DefaultErrorStrategy<'input,LambdaCalculusParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LambdaCalculusParser
pub trait LambdaCalculusParserContext<'input>:
	for<'x> Listenable<dyn LambdaCalculusListener<'input> + 'x > + 
	for<'x> Visitable<dyn LambdaCalculusVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LambdaCalculusParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn LambdaCalculusParserContext<'input> + 'input
where
    T: LambdaCalculusVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn LambdaCalculusVisitor<'input> + 'x))
    }
}

impl<'input> LambdaCalculusParserContext<'input> for TerminalNode<'input,LambdaCalculusParserContextType> {}
impl<'input> LambdaCalculusParserContext<'input> for ErrorNode<'input,LambdaCalculusParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn LambdaCalculusParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn LambdaCalculusListener<'input> + 'input{}

pub struct LambdaCalculusParserContextType;
antlr_rust::type_id!{LambdaCalculusParserContextType}

impl<'input> ParserNodeType<'input> for LambdaCalculusParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LambdaCalculusParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for LambdaCalculusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for LambdaCalculusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LambdaCalculusParserExt{
}

impl LambdaCalculusParserExt{
}


impl<'input> TokenAware<'input> for LambdaCalculusParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LambdaCalculusParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LambdaCalculusParserExt{
	fn get_grammar_file_name(&self) -> & str{ "LambdaCalculus.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn LambdaCalculusParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					0 => LambdaCalculusParser::<'input,I,_>::term_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> LambdaCalculusParser<'input, I, DefaultErrorStrategy<'input,LambdaCalculusParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn term_sempred(_localctx: Option<&TermContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
}
//------------------- term ----------------
#[derive(Debug)]
pub enum TermContextAll<'input>{
	ParensContext(ParensContext<'input>),
	ApplicationContext(ApplicationContext<'input>),
	AbstractionContext(AbstractionContext<'input>),
	VariableContext(VariableContext<'input>),
	LetContext(LetContext<'input>),
Error(TermContext<'input>)
}
antlr_rust::type_id!{TermContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TermContextAll<'input>{}

impl<'input> LambdaCalculusParserContext<'input> for TermContextAll<'input>{}

impl<'input> Deref for TermContextAll<'input>{
	type Target = dyn TermContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TermContextAll::*;
		match self{
			ParensContext(inner) => inner,
			ApplicationContext(inner) => inner,
			AbstractionContext(inner) => inner,
			VariableContext(inner) => inner,
			LetContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for TermContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for TermContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LambdaCalculusParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for TermContext<'input>{
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for TermContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::type_id!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn LambdaCalculusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
		TermContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TermContextAttrs<'input>: LambdaCalculusParserContext<'input> + BorrowMut<TermContextExt<'input>>{


}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

pub type ParensContext<'input> = BaseParserRuleContext<'input,ParensContextExt<'input>>;

pub trait ParensContextAttrs<'input>: LambdaCalculusParserContext<'input>{
	fn term(&self) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ParensContextAttrs<'input> for ParensContext<'input>{}

pub struct ParensContextExt<'input>{
	base:TermContextExt<'input>,
	pub inner: Option<Rc<TermContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ParensContextExt<'a>}

impl<'input> LambdaCalculusParserContext<'input> for ParensContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for ParensContext<'input>{
	fn enter(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parens(self);
	}
	fn exit(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.exit_parens(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for ParensContext<'input>{
	fn accept(&self,visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) {
		visitor.visit_parens(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for ParensContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for ParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for ParensContext<'input> {}

impl<'input> ParensContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::ParensContext(
				BaseParserRuleContext::copy_from(ctx,ParensContextExt{
        			inner:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ApplicationContext<'input> = BaseParserRuleContext<'input,ApplicationContextExt<'input>>;

pub trait ApplicationContextAttrs<'input>: LambdaCalculusParserContext<'input>{
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ApplicationContextAttrs<'input> for ApplicationContext<'input>{}

pub struct ApplicationContextExt<'input>{
	base:TermContextExt<'input>,
	pub left: Option<Rc<TermContextAll<'input>>>,
	pub right: Option<Rc<TermContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{ApplicationContextExt<'a>}

impl<'input> LambdaCalculusParserContext<'input> for ApplicationContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for ApplicationContext<'input>{
	fn enter(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_application(self);
	}
	fn exit(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.exit_application(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for ApplicationContext<'input>{
	fn accept(&self,visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) {
		visitor.visit_application(self);
	}
}

impl<'input> CustomRuleContext<'input> for ApplicationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for ApplicationContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for ApplicationContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for ApplicationContext<'input> {}

impl<'input> ApplicationContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::ApplicationContext(
				BaseParserRuleContext::copy_from(ctx,ApplicationContextExt{
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AbstractionContext<'input> = BaseParserRuleContext<'input,AbstractionContextExt<'input>>;

pub trait AbstractionContextAttrs<'input>: LambdaCalculusParserContext<'input>{
	fn term(&self) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token VARIABLE in current rule
	fn VARIABLE_all(&self) -> Vec<Rc<TerminalNode<'input,LambdaCalculusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token VARIABLE, starting from 0.
	/// Returns `None` if number of children corresponding to token VARIABLE is less or equal than `i`.
	fn VARIABLE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LambdaCalculusParserContextType>>> where Self:Sized{
		self.get_token(VARIABLE, i)
	}
}

impl<'input> AbstractionContextAttrs<'input> for AbstractionContext<'input>{}

pub struct AbstractionContextExt<'input>{
	base:TermContextExt<'input>,
	pub VARIABLE: Option<TokenType<'input>>,
	pub variables:Vec<TokenType<'input>>,
	pub abs_body: Option<Rc<TermContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{AbstractionContextExt<'a>}

impl<'input> LambdaCalculusParserContext<'input> for AbstractionContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for AbstractionContext<'input>{
	fn enter(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_abstraction(self);
	}
	fn exit(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.exit_abstraction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for AbstractionContext<'input>{
	fn accept(&self,visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) {
		visitor.visit_abstraction(self);
	}
}

impl<'input> CustomRuleContext<'input> for AbstractionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for AbstractionContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for AbstractionContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for AbstractionContext<'input> {}

impl<'input> AbstractionContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::AbstractionContext(
				BaseParserRuleContext::copy_from(ctx,AbstractionContextExt{
					VARIABLE:None, 
        			variables:Vec::new(), 
        			abs_body:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

pub trait VariableContextAttrs<'input>: LambdaCalculusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VARIABLE
	/// Returns `None` if there is no child corresponding to token VARIABLE
	fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,LambdaCalculusParserContextType>>> where Self:Sized{
		self.get_token(VARIABLE, 0)
	}
}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

pub struct VariableContextExt<'input>{
	base:TermContextExt<'input>,
	pub variable: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{VariableContextExt<'a>}

impl<'input> LambdaCalculusParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for VariableContext<'input>{
	fn enter(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variable(self);
	}
	fn exit(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.exit_variable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for VariableContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for VariableContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for VariableContext<'input> {}

impl<'input> VariableContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::VariableContext(
				BaseParserRuleContext::copy_from(ctx,VariableContextExt{
					variable:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LetContext<'input> = BaseParserRuleContext<'input,LetContextExt<'input>>;

pub trait LetContextAttrs<'input>: LambdaCalculusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VARIABLE
	/// Returns `None` if there is no child corresponding to token VARIABLE
	fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,LambdaCalculusParserContextType>>> where Self:Sized{
		self.get_token(VARIABLE, 0)
	}
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> LetContextAttrs<'input> for LetContext<'input>{}

pub struct LetContextExt<'input>{
	base:TermContextExt<'input>,
	pub variable: Option<TokenType<'input>>,
	pub assignment: Option<Rc<TermContextAll<'input>>>,
	pub body: Option<Rc<TermContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::type_id!{LetContextExt<'a>}

impl<'input> LambdaCalculusParserContext<'input> for LetContext<'input>{}

impl<'input,'a> Listenable<dyn LambdaCalculusListener<'input> + 'a> for LetContext<'input>{
	fn enter(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_let(self);
	}
	fn exit(&self,listener: &mut (dyn LambdaCalculusListener<'input> + 'a)) {
		listener.exit_let(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LambdaCalculusVisitor<'input> + 'a> for LetContext<'input>{
	fn accept(&self,visitor: &mut (dyn LambdaCalculusVisitor<'input> + 'a)) {
		visitor.visit_let(self);
	}
}

impl<'input> CustomRuleContext<'input> for LetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LambdaCalculusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for LetContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for LetContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for LetContext<'input> {}

impl<'input> LetContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::LetContext(
				BaseParserRuleContext::copy_from(ctx,LetContextExt{
					variable:None, 
        			assignment:None, body:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LambdaCalculusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		self.term_rec(0)
	}

	fn term_rec(&mut self, _p: isize)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 0, RULE_term, _p);
	    let mut _localctx: Rc<TermContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 0;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(23);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 VARIABLE 
				=> {
					{
					let mut tmp = VariableContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(3);
					let tmp = recog.base.match_token(VARIABLE,&mut recog.err_handler)?;
					if let TermContextAll::VariableContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.variable = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 T__0 
				=> {
					{
					let mut tmp = AbstractionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(4);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					recog.base.set_state(6); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(5);
						let tmp = recog.base.match_token(VARIABLE,&mut recog.err_handler)?;
						if let TermContextAll::AbstractionContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
						ctx.VARIABLE = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						let temp = if let TermContextAll::AbstractionContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
						ctx.VARIABLE.clone().unwrap() } else {unreachable!("cant cast");} ;
						if let TermContextAll::AbstractionContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
						ctx.variables.push(temp); } else {unreachable!("cant cast");}  
						}
						}
						recog.base.set_state(8); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==VARIABLE) {break}
					}
					recog.base.set_state(10);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule term*/
					recog.base.set_state(11);
					let tmp = recog.term_rec(4)?;
					if let TermContextAll::AbstractionContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.abs_body = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 T__2 
				=> {
					{
					let mut tmp = LetContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(12);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					recog.base.set_state(13);
					let tmp = recog.base.match_token(VARIABLE,&mut recog.err_handler)?;
					if let TermContextAll::LetContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.variable = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(14);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					/*InvokeRule term*/
					recog.base.set_state(15);
					let tmp = recog.term_rec(0)?;
					if let TermContextAll::LetContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.assignment = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(16);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule term*/
					recog.base.set_state(17);
					let tmp = recog.term_rec(2)?;
					if let TermContextAll::LetContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.body = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 T__5 
				=> {
					{
					let mut tmp = ParensContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(19);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule term*/
					recog.base.set_state(20);
					let tmp = recog.term_rec(0)?;
					if let TermContextAll::ParensContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.inner = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(21);
					recog.base.match_token(T__6,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(29);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleLabeledAltStartAction*/
					let mut tmp = ApplicationContextExt::new(&**TermContextExt::new(_parentctx.clone(), _parentState));
					if let TermContextAll::ApplicationContext(ctx) = cast_mut::<_,TermContextAll >(&mut tmp){
						ctx.left = Some(_prevctx.clone());
					} else {unreachable!("cant cast");}
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_term);
					_localctx = tmp;
					recog.base.set_state(25);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					/*InvokeRule term*/
					recog.base.set_state(26);
					let tmp = recog.term_rec(4)?;
					if let TermContextAll::ApplicationContext(ctx) = cast_mut::<_,TermContextAll >(&mut _localctx){
					ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
					} 
				}
				recog.base.set_state(31);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x0b\x23\x04\x02\x09\x02\x03\x02\x03\x02\x03\x02\x03\x02\x06\x02\x09\x0a\
	\x02\x0d\x02\x0e\x02\x0a\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\
	\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x05\x02\x1a\x0a\
	\x02\x03\x02\x03\x02\x07\x02\x1e\x0a\x02\x0c\x02\x0e\x02\x21\x0b\x02\x03\
	\x02\x02\x03\x02\x03\x02\x02\x02\x02\x26\x02\x19\x03\x02\x02\x02\x04\x05\
	\x08\x02\x01\x02\x05\x1a\x07\x0a\x02\x02\x06\x08\x07\x03\x02\x02\x07\x09\
	\x07\x0a\x02\x02\x08\x07\x03\x02\x02\x02\x09\x0a\x03\x02\x02\x02\x0a\x08\
	\x03\x02\x02\x02\x0a\x0b\x03\x02\x02\x02\x0b\x0c\x03\x02\x02\x02\x0c\x0d\
	\x07\x04\x02\x02\x0d\x1a\x05\x02\x02\x06\x0e\x0f\x07\x05\x02\x02\x0f\x10\
	\x07\x0a\x02\x02\x10\x11\x07\x06\x02\x02\x11\x12\x05\x02\x02\x02\x12\x13\
	\x07\x07\x02\x02\x13\x14\x05\x02\x02\x04\x14\x1a\x03\x02\x02\x02\x15\x16\
	\x07\x08\x02\x02\x16\x17\x05\x02\x02\x02\x17\x18\x07\x09\x02\x02\x18\x1a\
	\x03\x02\x02\x02\x19\x04\x03\x02\x02\x02\x19\x06\x03\x02\x02\x02\x19\x0e\
	\x03\x02\x02\x02\x19\x15\x03\x02\x02\x02\x1a\x1f\x03\x02\x02\x02\x1b\x1c\
	\x0c\x05\x02\x02\x1c\x1e\x05\x02\x02\x06\x1d\x1b\x03\x02\x02\x02\x1e\x21\
	\x03\x02\x02\x02\x1f\x1d\x03\x02\x02\x02\x1f\x20\x03\x02\x02\x02\x20\x03\
	\x03\x02\x02\x02\x21\x1f\x03\x02\x02\x02\x05\x0a\x19\x1f";

