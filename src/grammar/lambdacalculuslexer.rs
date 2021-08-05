// Generated from LambdaCalculus.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const T__2:isize=3; 
	pub const T__3:isize=4; 
	pub const T__4:isize=5; 
	pub const T__5:isize=6; 
	pub const T__6:isize=7; 
	pub const VARIABLE:isize=8; 
	pub const WS:isize=9; 
	pub const SINGLE_LINE_COMMENT:isize=10; 
	pub const MULTI_LINE_COMMENT:isize=11;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;11] = [
		"T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "VARIABLE", "WS", 
		"SINGLE_LINE_COMMENT", "MULTI_LINE_COMMENT"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("'\u{03BB}'"), Some("'.'"), Some("'let'"), Some("'='"), Some("'in'"), 
		Some("'('"), Some("')'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
		None, None, None, None, None, None, None, None, Some("VARIABLE"), Some("WS"), 
		Some("SINGLE_LINE_COMMENT"), Some("MULTI_LINE_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

#[derive(Tid)]
pub struct LambdaCalculusLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,LambdaCalculusLexerActions,Input,LocalTokenFactory<'input>>,
}

impl<'input, Input:CharStream<From<'input> >> Deref for LambdaCalculusLexer<'input,Input>{
	type Target = BaseLexer<'input,LambdaCalculusLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for LambdaCalculusLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> LambdaCalculusLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "LambdaCalculusLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","2");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				LambdaCalculusLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> LambdaCalculusLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		LambdaCalculusLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct LambdaCalculusLexerActions {
}

impl LambdaCalculusLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,LambdaCalculusLexerActions,Input,LocalTokenFactory<'input>>> for LambdaCalculusLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> LambdaCalculusLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,LambdaCalculusLexerActions,Input,LocalTokenFactory<'input>>> for LambdaCalculusLexerActions{
}
impl<'input> TokenAware<'input> for LambdaCalculusLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for LambdaCalculusLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x0d\x50\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\
		\x04\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x03\x02\x03\x02\x03\x03\
		\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\
		\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\x09\x06\x09\x2c\x0a\x09\x0d\
		\x09\x0e\x09\x2d\x03\x0a\x06\x0a\x31\x0a\x0a\x0d\x0a\x0e\x0a\x32\x03\x0a\
		\x03\x0a\x03\x0b\x03\x0b\x07\x0b\x39\x0a\x0b\x0c\x0b\x0e\x0b\x3c\x0b\x0b\
		\x03\x0b\x05\x0b\x3f\x0a\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\
		\x0c\x07\x0c\x47\x0a\x0c\x0c\x0c\x0e\x0c\x4a\x0b\x0c\x03\x0c\x03\x0c\x03\
		\x0c\x03\x0c\x03\x0c\x04\x3a\x48\x02\x0d\x03\x03\x05\x04\x07\x05\x09\x06\
		\x0b\x07\x0d\x08\x0f\x09\x11\x0a\x13\x0b\x15\x0c\x17\x0d\x03\x02\x05\x06\
		\x02\x32\x3b\x43\x5c\x61\x61\x63\x7c\x05\x02\x0c\x0c\x0f\x0f\x22\x22\x04\
		\x03\x0c\x0c\x0f\x0f\x02\x53\x02\x03\x03\x02\x02\x02\x02\x05\x03\x02\x02\
		\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x02\x0b\x03\x02\x02\
		\x02\x02\x0d\x03\x02\x02\x02\x02\x0f\x03\x02\x02\x02\x02\x11\x03\x02\x02\
		\x02\x02\x13\x03\x02\x02\x02\x02\x15\x03\x02\x02\x02\x02\x17\x03\x02\x02\
		\x02\x03\x19\x03\x02\x02\x02\x05\x1b\x03\x02\x02\x02\x07\x1d\x03\x02\x02\
		\x02\x09\x21\x03\x02\x02\x02\x0b\x23\x03\x02\x02\x02\x0d\x26\x03\x02\x02\
		\x02\x0f\x28\x03\x02\x02\x02\x11\x2b\x03\x02\x02\x02\x13\x30\x03\x02\x02\
		\x02\x15\x36\x03\x02\x02\x02\x17\x42\x03\x02\x02\x02\x19\x1a\x07\u{3bd}\
		\x02\x02\x1a\x04\x03\x02\x02\x02\x1b\x1c\x07\x30\x02\x02\x1c\x06\x03\x02\
		\x02\x02\x1d\x1e\x07\x6e\x02\x02\x1e\x1f\x07\x67\x02\x02\x1f\x20\x07\x76\
		\x02\x02\x20\x08\x03\x02\x02\x02\x21\x22\x07\x3f\x02\x02\x22\x0a\x03\x02\
		\x02\x02\x23\x24\x07\x6b\x02\x02\x24\x25\x07\x70\x02\x02\x25\x0c\x03\x02\
		\x02\x02\x26\x27\x07\x2a\x02\x02\x27\x0e\x03\x02\x02\x02\x28\x29\x07\x2b\
		\x02\x02\x29\x10\x03\x02\x02\x02\x2a\x2c\x09\x02\x02\x02\x2b\x2a\x03\x02\
		\x02\x02\x2c\x2d\x03\x02\x02\x02\x2d\x2b\x03\x02\x02\x02\x2d\x2e\x03\x02\
		\x02\x02\x2e\x12\x03\x02\x02\x02\x2f\x31\x09\x03\x02\x02\x30\x2f\x03\x02\
		\x02\x02\x31\x32\x03\x02\x02\x02\x32\x30\x03\x02\x02\x02\x32\x33\x03\x02\
		\x02\x02\x33\x34\x03\x02\x02\x02\x34\x35\x08\x0a\x02\x02\x35\x14\x03\x02\
		\x02\x02\x36\x3a\x07\x25\x02\x02\x37\x39\x0b\x02\x02\x02\x38\x37\x03\x02\
		\x02\x02\x39\x3c\x03\x02\x02\x02\x3a\x3b\x03\x02\x02\x02\x3a\x38\x03\x02\
		\x02\x02\x3b\x3e\x03\x02\x02\x02\x3c\x3a\x03\x02\x02\x02\x3d\x3f\x09\x04\
		\x02\x02\x3e\x3d\x03\x02\x02\x02\x3f\x40\x03\x02\x02\x02\x40\x41\x08\x0b\
		\x02\x02\x41\x16\x03\x02\x02\x02\x42\x43\x07\x2a\x02\x02\x43\x44\x07\x2c\
		\x02\x02\x44\x48\x03\x02\x02\x02\x45\x47\x0b\x02\x02\x02\x46\x45\x03\x02\
		\x02\x02\x47\x4a\x03\x02\x02\x02\x48\x49\x03\x02\x02\x02\x48\x46\x03\x02\
		\x02\x02\x49\x4b\x03\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4b\x4c\x07\x2c\
		\x02\x02\x4c\x4d\x07\x2b\x02\x02\x4d\x4e\x03\x02\x02\x02\x4e\x4f\x08\x0c\
		\x02\x02\x4f\x18\x03\x02\x02\x02\x08\x02\x2d\x32\x3a\x3e\x48\x03\x08\x02\
		\x02";
