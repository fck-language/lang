use std::iter::FromIterator;
use regex::Regex;
use crate::prelude::LanguageRaw;

pub type Position = (usize, u16);

pub struct Token {
	ps: Position,
	pe: Position,
	tt: TokType
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokType<T> where T: ToString {
	/// Integer literal
	Int(u64),
	/// Float literal
	Float(f64),
	/// Boolean literal
	Bool(bool),
	/// String literal
	String(T),
	/// Plus operator
	Plus,
	/// Minus operator
	Minus,
	/// Modulus operator
	Mod,
	/// Multiply operator
	Mult,
	/// Divide operator
	Div,
	/// Floor divide operator
	FDiv,
	/// Power operator
	Pow,
	/// Increment
	Increment,
	/// Decrement
	Decrement,
	/// Left parentheses (
	LParen,
	/// Right parentheses )
	RParen,
	/// Left curly parentheses {
	LParenCurly,
	/// Right curly parentheses }
	RParenCurly,
	/// Left square parentheses [
	LParenSquare,
	/// Right square parentheses ]
	RParenSquare,
	/// At identifier
	Label(T),
	/// Boolean negation
	Not,
	/// Colon
	Colon,
	/// Identifier. Holds identifier string and language key
	Identifier(T, T),
	/// Keyword
	Keyword(u8, u8),
	/// Type index
	Type(u16),
	/// Question mark
	QuestionMark,
	/// Dot
	Dot,
	/// Equals operator
	Eq,
	/// Not equals operator
	NE,
	/// Less than operator
	LT,
	/// Greater than operator
	GT,
	/// Less than or equals operator
	LTE,
	/// Greater than or equals operator
	GTE,
	/// Comma
	Comma,
	/// New line
	Newline,
	/// Variable assignment
	Set,
	/// Add value to current variable
	SetPlus,
	/// Subtract value from current variable
	SetMinus,
	/// Modulus current variable
	SetMod,
	/// Multiply variable
	SetMult,
	/// Divide variable
	SetDiv,
	/// Floor divide variable
	SetFDiv,
	/// Power current variable
	SetPow,
	/// Comment token. Used exclusively by the translator to return comments
	Comment(T, T),
	/// Space. Removed by a filter but required for
	Space
}

/// # Lexing builder trait
/// Used by the lexer to convert a matched token giving a `(u8, u8)` into a `Self` instance.
/// Also used when turning a [`LanguageRaw`] into a lexing transition table to convert `Self` into
/// its matching `(u8, u8)` tuple
pub trait Builder {
	/// Make a new `Self` instance from a given token type (`tt`) and token data (`td`). Also
	/// provides the matched section (`matcher`) and [`LanguageRaw`] for use (for example
	fn build(tt: u8, td: u8, matcher: Vec<char>, l: &LanguageRaw<'_>) -> Self;
	fn inverse_build(self) -> (u8, u8);
}

impl Builder for TokType<String> {
	fn build(tt: u8, td: u8, matcher: Vec<u8>, l: &LanguageRaw<'_>) -> Self {
		todo!()
	}
	// fn build(self, data: Vec<char>) -> Self {
	// 	match self {
	// 		TokType::Int(_) => {
	// 			TokType::Int(String::from_iter(data.iter()).parse().unwrap())
	// 		}
	// 		TokType::Float(_) => {
	// 			TokType::Float(String::from_iter(data.iter()).parse().unwrap())
	// 		}
	// 		TokType::String(_) => {
	// 			let r = 1..data.len() - 1;
	// 			let s = String::from_iter(data[r].iter())
	// 				.replace("\\t", "\t")
	// 				.replace("\\n", "\n")
	// 				.replace("\\r", "\r");
	// 			let s = Regex::new("\\\\(?P<n>^\\\\)").unwrap().replace(&*s, "$n");
	// 			TokType::String(s.to_string())
	// 		}
	// 		TokType::Label(_) => {
	// 			TokType::Label(String::from_iter(data.iter()))
	// 		}
	// 		TokType::Identifier(l, _) => {
	// 			let s = String::from_iter(data.iter());
	// 			if let Some(p) = s.chars().position(|t| t == ':') {
	// 				TokType::Identifier(s[..p].to_string(), s[p + 1..].to_string())
	// 			} else {
	// 				TokType::Identifier(l, s)
	// 			}
	// 		}
	// 		TokType::Comment(l, _) => {
	// 			TokType::Comment(l, String::from_iter(data.iter()))
	// 		}
	// 		_ => self
	// 	}
	// }
	fn inverse_build(self) -> (u8, u8) {
		match self {
			TokType::Int(_) => (0, 0),
			TokType::Float(_) => (1, 0),
			TokType::Bool(_) => (2, 0),
			TokType::String(_) => (3, 0),
			TokType::Plus => (4, 0),
			TokType::Minus => (5, 0),
			TokType::Mod => {}
			TokType::Mult => {}
			TokType::Div => {}
			TokType::FDiv => {}
			TokType::Pow => {}
			TokType::Increment => {}
			TokType::Decrement => {}
			TokType::LParen => {}
			TokType::RParen => {}
			TokType::LParenCurly => {}
			TokType::RParenCurly => {}
			TokType::LParenSquare => {}
			TokType::RParenSquare => {}
			TokType::Label(_) => {}
			TokType::Not => {}
			TokType::Colon => {}
			TokType::Identifier(_, _) => {}
			TokType::Keyword(_) => {}
			TokType::Type(_) => {}
			TokType::QuestionMark => {}
			TokType::Dot => {}
			TokType::Eq => {}
			TokType::NE => {}
			TokType::LT => {}
			TokType::GT => {}
			TokType::LTE => {}
			TokType::GTE => {}
			TokType::Comma => {}
			TokType::Newline => {}
			TokType::Set => {}
			TokType::SetPlus => {}
			TokType::SetMinus => {}
			TokType::SetMod => {}
			TokType::SetMult => {}
			TokType::SetDiv => {}
			TokType::SetFDiv => {}
			TokType::SetPow => {}
			TokType::Comment(_, _) => {}
			TokType::Space => {}
		}
	}
}

impl Token {
	fn from(ps: Position, pe: Position, tt: u8, td: u8) -> Self {
		todo!()
	}
}
