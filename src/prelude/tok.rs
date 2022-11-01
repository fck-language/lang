use std::iter::FromIterator;
use regex::Regex;

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
	Keyword(u16),
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

pub trait Builder<T> {
	fn build(self, data: Vec<T>) -> Self;
}

impl Builder<char> for TokType<String> {
	fn build(self, data: Vec<char>) -> Self {
		match self {
			TokType::Int(_) => {
				TokType::Int(String::from_iter(data.iter()).parse().unwrap())
			}
			TokType::Float(_) => {
				TokType::Float(String::from_iter(data.iter()).parse().unwrap())
			}
			TokType::String(_) => {
				let r = 1..data.len() - 1;
				let s = String::from_iter(data[r].iter())
					.replace("\\t", "\t")
					.replace("\\n", "\n")
					.replace("\\r", "\r");
				let s = Regex::new("\\\\(?P<n>^\\\\)").unwrap().replace(&*s, "$n");
				TokType::String(s.to_string())
			}
			TokType::Label(_) => {
				TokType::Label(String::from_iter(data.iter()))
			}
			TokType::Identifier(l, _) => {
				let s = String::from_iter(data.iter());
				if let Some(p) = s.chars().position(|t| t == ':') {
					TokType::Identifier(s[..p].to_string(), s[p + 1..].to_string())
				} else {
					TokType::Identifier(l, s)
				}
			}
			TokType::Comment(l, _) => {
				TokType::Comment(l, String::from_iter(data.iter()))
			}
			_ => self
		}
	}
}
