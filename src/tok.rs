//! # Tokens
//!
//! This module contains all the token type. It also contains two position structs used in parsing

use std::fmt::{Debug, Formatter};
use cflp::NodeData;
use num_bigint::BigUint;
use lang_inner::{Digits, LanguageRaw};
use lang_macros::doc_see;

/// # Intermediary position struct
///
/// Used to hold required positional data when lexing, and can be transformed into a [`Position`]
/// with the [`finish`](Self::finish) method
#[derive(Clone)]
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct RunningPosition {
	ln: usize,
	col: usize,
	pub previous: u8,
}

/// # Position data
///
/// Holds positional data on tokens and AST nodes
#[derive(Copy, Clone)]
pub struct Position {
	pub ln: usize,
	pub col: usize,
}

impl Default for Position {
	fn default() -> Self {
		Self {
			ln: 0, col: 0
		}
	}
}

#[cfg(any(test, debug_assertions))]
impl Debug for Position {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.ln, self.col)
	}
}

impl RunningPosition {
	/// Make a new instance of `Self`. Sets the [line](Self::ln), [column](Self::col), and
	/// [previous byte](Self::previous) to 0
	pub fn new() -> Self {
		Self {
			ln: 0,
			col: 0,
			previous: 0,
		}
	}
	
	/// Advance the position on a given byte
	pub fn advance(&mut self, b: u8) {
		if b == b'\n' {
			self.ln += 1;
			self.col = 0
		} else {
			self.col += 1
		}
		if b == b'\r' && self.previous == b'\n' {
			self.col -= 1
		}
		self.previous = b
	}
	
	/// Make a new [`Position`] instance from `Self`
	pub fn finish(&self) -> Position {
		Position {
			ln: self.ln,
			col: self.col,
		}
	}
}

/// # Token
///
/// Holds a starting end ending [`Position`] as well as a [token type](PreTokType)
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token {
	/// Starting position
	pub ps: Position,
	/// Ending position
	pub pe: Position,
	/// Token type
	pub tt: TokType,
}

/// # PreToken
///
/// Holds a starting end ending [`Position`] as well as a [pre-token type](PreTokType).
///
/// Converted into a [`Token`] after lexing is complete
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct PreToken<'a> {
	/// Starting position
	pub ps: Position,
	/// Ending position
	pub pe: Position,
	/// Token type
	pub tt: PreTokType<'a>,
}

impl From<&PreToken<'_>> for Token {
	fn from(tok: &PreToken<'_>) -> Self {
		Self {
			ps: tok.ps, pe: tok.pe, tt: TokType::from(tok.tt.clone())
		}
	}
}

impl PartialEq for Token {
	fn eq(&self, other: &Self) -> bool {
		self.tt == other.tt
	}
}

impl PartialEq<TokType> for Token {
	fn eq(&self, other: &TokType) -> bool {
		self.tt == *other
	}
}

impl PartialEq<TokType> for &Token {
	fn eq(&self, other: &TokType) -> bool {
		self.tt == *other
	}
}

impl Into<TokType> for &Token {
	fn into(self) -> TokType {
		self.tt.clone()
	}
}

impl NodeData<Position> for Token {
	fn start(&self) -> Position {
		self.ps
	}
	
	fn end(&self) -> Position {
		self.pe
	}
}

/// # Token type
///
/// Final token type enum. These are from the [`PreTokType`] and are converted once lexing is done.
/// This is done since `PreTokType` does not convert raw bytes into usable data for speed (e.g.
/// numbers are left as matched bytes). This is then converted into usable data once lexing is done
/// to improve speed.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum TokType {
	/// Integer literal
	/// - `tt=1`
	/// - `td=2..=5`
	///
	/// Also see the `digits` module of [tables](lang_inner::tables) for `td` values
	Int(BigUint),
	/// Float literal
	/// - `tt=1`
	/// - `td` is todo
	Float(f64),
	/// Boolean literal
	/// - `tt=2`
	/// - `td=0` for `false` and `td=1` for `true`
	Bool(bool),
	/// String literal
	/// - `tt=3`
	String(Vec<u8>),
	/// Character literal
	/// - `tt=3`
	Char(char),
	/// Operator such as `+` or `**`
	/// - `tt=4`
	/// - `td=0..=8` (See [`Op::new`] for specific `td` values)
	Op(Op),
	/// Comparison such as `==` or `!=`
	/// - `tt=4`
	/// - `td=0..=5` (See [`Cmp::new`] for specific `td` values)
	Cmp(Cmp),
	/// Increment
	/// - `tt=4`
	/// - `td=6`
	Increment,
	/// Decrement
	/// - `tt=4`
	/// - `td=7`
	Decrement,
	/// Left parentheses (
	/// - `tt=5`
	/// - `td=0`
	LParen,
	/// Right parentheses )
	/// - `tt=5`
	/// - `td=1`
	RParen,
	/// Left curly parentheses {
	/// - `tt=5`
	/// - `td=2`
	LParenCurly,
	/// Right curly parentheses }
	/// - `tt=5`
	/// - `td=3`
	RParenCurly,
	/// Left square parentheses [
	/// - `tt=5`
	/// - `td=4`
	LParenSquare,
	/// Right square parentheses ]
	/// - `tt=5`
	/// - `td=5`
	RParenSquare,
	/// Boolean negation
	/// - `tt=4`
	/// - `td=8`
	Not,
	/// Colon
	/// - `tt=4`
	/// - `td=9`
	Colon,
	/// Identifier. Holds identifier string and language key
	/// - `td=10`
	Identifier(String, Vec<u8>),
	/// Keyword
	/// - `td=8`
	Keyword(u8),
	/// Question mark
	/// - `tt=4`
	/// - `td=10`
	QuestionMark,
	/// Dot
	/// - `tt=4`
	/// - `td=11`
	Dot,
	/// Set/modifier set operator such as `+=` or `=`
	/// - `tt=7`
	/// - `td=0` for `None` and `td=1..6` for `Some(Op)`
	///   See [`Op::new_self`] for specific `td` values (note the used `td` value here is one more
	///   than used for `Op::new_self`)
	Set(Option<Op>),
	/// Comment token. Used exclusively by the translator to return comments
	/// - `tt=254`
	Comment(String, Vec<u8>),
}

impl Default for TokType {
	fn default() -> Self {
		Self::LParen
	}
}

/// # Intermediary Token types
/// Each variant matches a variant in [`TokType`] that it's converted to once lexing is complete
#[allow(missing_docs)]
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[doc_see(TokType)]
pub(crate) enum PreTokType<'a> {
	/// Saves the matched digit and the base of the digit
	Int(Vec<u8>, u8, &'a LanguageRaw<'a>),
	Float(f64),
	Bool(bool),
	String(Vec<u8>),
	Char(char),
	Op(Op),
	Cmp(Cmp),
	Increment,
	Decrement,
	LParen,
	RParen,
	LParenCurly,
	RParenCurly,
	LParenSquare,
	RParenSquare,
	Not,
	Colon,
	Identifier(String, Vec<u8>),
	Keyword(u8),
	QuestionMark,
	Dot,
	Set(Option<Op>),
	Comment(String, Vec<u8>),
}

impl<'a> From<PreTokType<'a>> for TokType {
	fn from(value: PreTokType<'a>) -> Self {
		match value {
			PreTokType::Int(matched, 10, l) => TokType::Int(new_biguint(matched, 10, &l.keywords.digits)),
			PreTokType::Int(matched, base, l) => {
				let prefixes = match l.keywords.digits {
					Digits::Short { u8arrays, .. } => u8arrays[..4].iter().map(|(_, t)| 4 - *t as usize).collect::<Vec<_>>(),
					Digits::Long { u8arrays, .. } => u8arrays[..4].iter().map(|(_, t)| 4 - *t as usize).collect::<Vec<_>>(),
				};
				match base {
					2 => TokType::Int(new_biguint(matched[prefixes[3] + prefixes[0]..].to_vec(), 2, &l.keywords.digits)),
					16 => TokType::Int(new_biguint(matched[prefixes[3] + prefixes[1]..].to_vec(), 16, &l.keywords.digits)),
					8 => TokType::Int(new_biguint(matched[prefixes[3] + prefixes[2]..].to_vec(), 8, &l.keywords.digits)),
					_ => unreachable!()
				}
			}
			PreTokType::Float(f) => TokType::Float(f),
			PreTokType::Bool(a) => TokType::Bool(a),
			PreTokType::String(a) => TokType::String(a),
			PreTokType::Char(a) => TokType::Char(a),
			PreTokType::Op(a) => TokType::Op(a),
			PreTokType::Cmp(a) => TokType::Cmp(a),
			PreTokType::Increment => TokType::Increment,
			PreTokType::Decrement => TokType::Decrement,
			PreTokType::LParen => TokType::LParen,
			PreTokType::RParen => TokType::RParen,
			PreTokType::LParenCurly => TokType::LParenCurly,
			PreTokType::RParenCurly => TokType::RParenCurly,
			PreTokType::LParenSquare => TokType::LParenSquare,
			PreTokType::RParenSquare => TokType::RParenSquare,
			PreTokType::Not => TokType::Not,
			PreTokType::Colon => TokType::Colon,
			PreTokType::Identifier(a, b) => TokType::Identifier(a, b),
			PreTokType::Keyword(a) => TokType::Keyword(a),
			PreTokType::QuestionMark => TokType::QuestionMark,
			PreTokType::Dot => TokType::Dot,
			PreTokType::Set(a) => TokType::Set(a),
			PreTokType::Comment(a, b) => TokType::Comment(a, b),
		}
	}
}

/// # Operator token type
///
/// An operator that can be used with an assignment. For example, this includes [`+`](Self::Plus)
/// since we can have `+` and `+=`, but does not include [`++`](PreTokType::Increment) because we cannot
/// have `++=`
#[derive(PartialEq, Clone)]
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum Op {
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
	/// Power operator
	Pow,
}

/// # Comparison token type
///
/// Comparison operations such as [`==`](Self::Eq) or [`<`](Self::LT)
#[derive(PartialEq, Clone)]
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum Cmp {
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
	/// Token for any comparison operator
	Any
}

impl Default for Cmp {
	fn default() -> Self {
		Self::Any
	}
}

/// Make a new [`BigUint`] from a matched digit, base, and digits
fn new_biguint(matched: Vec<u8>, base: u8, d: &Digits) -> BigUint {
	let mut out = BigUint::new(Vec::new());
	let separated = d.separate(base as usize, matched);
	match base {
		2 => for k in separated {
			out <<= 1u8;
			out += k
		}
		8 => for k in separated {
			out <<= 3u8;
			out += k
		}
		16 => for k in separated {
			out <<= 4u8;
			out += k
		}
		10 => for k in separated {
			out *= 10u8;
			out += k
		}
		_ => unreachable!()
	}
	out
}

impl<'a> PreTokType<'a> {
	fn new(tt: u8, td: u8, matcher: Vec<u8>, l: &'a LanguageRaw<'a>) -> Self {
		match tt {
			1 => match td {
				0 => PreTokType::Bool(true),
				1 => PreTokType::Bool(false),
				2 => PreTokType::Int(matcher, 10, l),
				3 => PreTokType::Int(matcher, 2, l),
				4 => PreTokType::Int(matcher, 16, l),
				5 => PreTokType::Int(matcher, 8, l),
				6 => PreTokType::Float(0.),
				// string and char are only ever manually constructed
				_ => unreachable!()
			}
			2 => Op::new(td),
			3 => PreTokType::Cmp(Cmp::new(td)),
			4 => match td {
				0 => PreTokType::LParen,
				1 => PreTokType::RParen,
				2 => PreTokType::LParenSquare,
				3 => PreTokType::RParenSquare,
				// '{' and '}' are only manually constructed
				_ => unreachable!(),
			},
			5 => match td {
				255 => PreTokType::Set(None),
				_ => PreTokType::Set(Some(Op::new_self(td))),
			},
			6 => PreTokType::Keyword(td),
			7 => PreTokType::Identifier(l.name.1.to_string(), matcher),
			255 => PreTokType::Comment(l.name.1.to_string(), matcher),
			t => unreachable!("TT={}", t),
		}
	}
}

impl Op {
	/// Make a new operator (either `TokType::Op(Self)` or other `TokType` operator) from a `td` value
	fn new<'a>(td: u8) -> PreTokType<'a> {
		match td {
			0..=5 => PreTokType::Op(Op::new_self(td)),
			6 => PreTokType::Increment,
			7 => PreTokType::Decrement,
			8 => PreTokType::Not,
			9 => PreTokType::Colon,
			10 => PreTokType::QuestionMark,
			11 => PreTokType::Dot,
			_ => unreachable!(),
		}
	}
	
	/// Make a new `Op` instance
	///
	/// | `td` | Variant |
	/// |---|---|
	/// | 0 | [`Op::Plus`] |
	/// | 1 | [`Op::Minus`] |
	/// | 2 | [`Op::Mod`] |
	/// | 3 | [`Op::Mult`] |
	/// | 4 | [`Op::Div`] |
	/// | 5 | [`Op::Pow`] |
	fn new_self(td: u8) -> Self {
		match td {
			0 => Op::Plus,
			1 => Op::Minus,
			2 => Op::Mod,
			3 => Op::Mult,
			4 => Op::Div,
			5 => Op::Pow,
			_ => unreachable!(),
		}
	}
}

impl Cmp {
	/// Make a new instance from a `td` value
	fn new(td: u8) -> Self {
		match td {
			0 => Cmp::Eq,
			1 => Cmp::NE,
			2 => Cmp::LT,
			3 => Cmp::GT,
			4 => Cmp::LTE,
			5 => Cmp::GTE,
			_ => unreachable!(),
		}
	}
}

impl<'a> PreToken<'a> {
	#[inline]
	pub(crate) fn new(ps: Position, pe: Position, tt: u8, td: u8, matcher: Vec<u8>, l: &'a LanguageRaw<'a>) -> Self {
		Self {
			ps, pe,
			tt: PreTokType::new(tt, td, matcher, l),
		}
	}
}
