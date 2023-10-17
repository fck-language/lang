//! # Tokens
//!
//! This module contains all the token type. It also contains two position structs used in parsing

use std::fmt::{Debug, Error, Formatter};
use cflp::NodeData;
use num_bigint::BigUint;
use lang_inner::{Digits, LanguageRaw};
use lang_macros::doc_see;

pub(crate) mod consts {
	pub const TAB: u8 = 9;
	pub const NEWLINE: u8 = 10;
	pub const SEMICOLON: u8 = 59;
	pub const SPACE: u8 = 32;
	pub const EXCLAMATION_MARK: u8 = 33;
	/// Open curly brace `{`
	pub const OCB: u8 = 123;
	/// Closed curly brace `}`
	pub const CCB: u8 = 125;
	pub const FORWARD_SLASH: u8 = 92;
	pub const STAR: u8 = 42;
	pub const DOUBLE_QUOTE: u8 = 34;
	pub const SINGLE_QUOTE: u8 = 39;
	pub const CARRIAGE_RETURN: u8 = 13;
}

/// # Intermediary position struct
///
/// Used to hold required positional data when lexing, and can be transformed into a [`Position`]
/// with the [`finish`](Self::finish) method
#[derive(Copy, Clone)]
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct RunningPosition {
	ln: usize,
	col: usize,
	pub previous: u8,
}

/// # Position data
///
/// Holds positional data on tokens and AST nodes
#[derive(Copy, Clone, PartialEq)]
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
		if b == consts::NEWLINE {
			self.ln += 1;
			self.col = 0
		} else {
			self.col += 1
		}
		if b == consts::CARRIAGE_RETURN && self.previous == consts::NEWLINE {
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
#[derive(Clone, PartialEq)]
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

impl NodeData<Position> for &Token {
	fn start(&self) -> Position { self.ps }
	fn end(&self) -> Position { self.pe }
}

/// # Token type
///
/// Final token type enum. These are from the [`PreTokType`] and are converted once lexing is done.
/// This is done since `PreTokType` does not convert raw bytes into usable data for speed (e.g.
/// numbers are left as matched bytes). This is then converted into usable data once lexing is done
/// to improve speed.
#[derive(PartialEq, Clone)]
pub enum TokType {
	/// Integer literal
	/// - `tt=1`
	/// - `td=2..=5`
	///
	/// See the `digits` module of [tables](lang_inner::tables) for `td` values
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
	ControlKeyword(ControlKeyword),
	DataKeyword(DataKeyword),
	PrimitiveKeyword(PrimitiveKeyword),
	/// Question mark
	/// - `tt=2`
	/// - `td=10`
	QuestionMark,
	/// Dot
	/// - `tt=2`
	/// - `td=11`
	Dot,
	/// Dot
	/// - `tt=2`
	/// - `td=12`
	Comma,
	/// Dot
	/// - `tt=2`
	/// - `td=13`
	At,
	/// Dot
	/// - `tt=2`
	/// - `td=14..=15`
	Arrow(Arrow),
	/// Newline token
	NewLine(NewLine),
	/// Set/modifier set operator such as `+=` or `=`
	/// - `tt=5`
	/// - `td=0` for `None` and `td=1..6` for `Some(Op)`
	///   See [`Op::new_self`] for specific `td` values (note the used `td` value here is one more
	///   than used for `Op::new_self`)
	Set(Option<Op>),
	/// Comment token. Used exclusively by the translator to return comments
	/// - `tt=255`
	Comment(String, Vec<u8>),
}

#[cfg(debug_assertions)]
impl Debug for TokType {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
		match self {
			Self::Int(i) => write!(f, "Int({})", i),
			Self::Float(i) => write!(f, "Float({})", i),
			Self::Bool(b) => write!(f, "Bool({})", b),
			Self::String(i) => write!(f, "String({:?})", String::from_utf8(i.clone()).unwrap_or(format!("{:?}", i))),
			Self::Char(c) => write!(f, "Char({})", c),
			Self::Op(op) => write!(f, "Op({:?})", op),
			Self::Cmp(cmp) => write!(f, "Cmp({:?})", cmp),
			Self::Increment => write!(f, "Increment"),
			Self::Decrement => write!(f, "Decrement"),
			Self::LParen => write!(f, "LParen"),
			Self::RParen => write!(f, "RParen"),
			Self::LParenCurly => write!(f, "LParenCurly"),
			Self::RParenCurly => write!(f, "RParenCurly"),
			Self::LParenSquare => write!(f, "LParenSquare"),
			Self::RParenSquare => write!(f, "RParenSquare"),
			Self::Not => write!(f, "Not"),
			Self::Colon => write!(f, "Colon"),
			Self::Identifier(lang, i) => write!(
				f, "Identifier({})",
				String::from_utf8(i.clone())
					.map(|id| format!("{}:{}", lang, id))
					.unwrap_or(format!("{}, {:?}", lang, i))
			),
			Self::ControlKeyword(ck) => write!(f, "ControlKeyword({:?})", ck),
			Self::DataKeyword(dk) => write!(f, "DataKeyword({:?})", dk),
			Self::PrimitiveKeyword(pk) => write!(f, "PrimitiveKeyword({:?})", pk),
			Self::QuestionMark => write!(f, "QuestionMark"),
			Self::Dot => write!(f, "Dot"),
			Self::Comma => write!(f, "Comma"),
			Self::At => write!(f, "At"),
			Self::Arrow(arr) => write!(f, "Arrow({:?})", arr),
			Self::NewLine(NewLine::Explicit) => write!(f, "Explicit newline"),
			Self::NewLine(NewLine::Implicit) => write!(f, "Implicit newline"),
			Self::Set(op) => write!(f, "Set({:?})", op),
			Self::Comment(lang, c) => write!(
				f, "Identifier({})",
				String::from_utf8(c.clone())
					.map(|id| format!("{}:{}", lang, id))
					.unwrap_or(format!("{}, {:?}", lang, c))
			)
		}
	}
}

impl Default for TokType {
	fn default() -> Self { Self::LParen }
}

impl Default for Op {
	fn default() -> Self { Self::Any }
}

impl Default for Cmp {
	fn default() -> Self { Self::Any }
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
	ControlKeyword(ControlKeyword),
	DataKeyword(DataKeyword),
	PrimitiveKeyword(PrimitiveKeyword),
	QuestionMark,
	Dot,
	Comma,
	At,
	Arrow(Arrow),
	NewLine(NewLine),
	Set(Option<Op>),
	Comment(String, Vec<u8>),
}

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum NewLine {
	/// Explicit newline from a `;`
	Explicit,
	/// Explicit newline from a `\n`
	Implicit
}

#[derive(Copy, Clone, PartialEq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ControlKeyword {
	KSet,
	KAnd,
	KOr,
	KNot,
	KIf,
	KElse,
	KMatch,
	KRepeat,
	KFor,
	KIn,
	KTo,
	KAs,
	KWhile,
	KFn,
	KReturn,
	KContinue,
	KBreak,
	KWhere
}

impl From<u8> for ControlKeyword {
	fn from(value: u8) -> Self {
		[
			Self::KSet, Self::KAnd, Self::KOr, Self::KNot, Self::KIf, Self::KElse,
			Self::KMatch, Self::KRepeat, Self::KFor, Self::KIn, Self::KTo, Self::KAs,
			Self::KWhile, Self::KFn, Self::KReturn, Self::KContinue, Self::KBreak, Self::KWhere
		][value as usize]
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DataKeyword {
	KStruct,
	KProperties,
	KEnum,
	KVariants,
	KSelf,
	KSSelf,
	KExtension,
	KExtend
}

impl From<u8> for DataKeyword {
	fn from(value: u8) -> Self {
		[
			Self::KStruct, Self::KProperties, Self::KEnum, Self::KVariants,
			Self::KSelf, Self::KSSelf, Self::KExtension, Self::KExtend
		][value as usize]
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrimitiveKeyword {
	KInt,
	KUint,
	KDint,
	KUdint,
	KFloat,
	KBfloat,
	KStr,
	KChar,
	KList,
	KBool
}

impl From<u8> for PrimitiveKeyword {
	fn from(value: u8) -> Self {
		[
			Self::KInt, Self::KUint, Self::KDint, Self::KUdint, Self::KFloat,
			Self::KBfloat, Self::KStr, Self::KChar, Self::KList, Self::KBool
		][value as usize]
	}
}

impl<'a> From<PreTokType<'a>> for TokType {
	fn from(value: PreTokType<'a>) -> Self {
		match value {
			PreTokType::Int(matched, 10, l) => TokType::Int(new_biguint(matched, 10, &l.keywords.digits)),
			PreTokType::Int(matched, base, l) => {
				let prefixes = match &l.keywords.digits {
					Digits::Short(t) => [t.oct_pre_u8, t.oct_pre_u8, t.oct_pre_u8, t.u8arrays[0]].map(|(_, t)| 4 - t as usize).to_vec(),
					Digits::Long(t) => [t.oct_pre_u8, t.oct_pre_u8, t.oct_pre_u8, t.u8arrays[0]].map(|(_, t)| 4 - t as usize).to_vec(),
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
			PreTokType::ControlKeyword(a) => TokType::ControlKeyword(a),
			PreTokType::DataKeyword(a) => TokType::DataKeyword(a),
			PreTokType::PrimitiveKeyword(a) => TokType::PrimitiveKeyword(a),
			PreTokType::QuestionMark => TokType::QuestionMark,
			PreTokType::Dot => TokType::Dot,
			PreTokType::Comma => TokType::Comma,
			PreTokType::At => TokType::At,
			PreTokType::Arrow(a) => TokType::Arrow(a),
			PreTokType::NewLine(a) => TokType::NewLine(a),
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
	/// Any operator
	Any
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

/// # Arrow types
///
/// Arrow tokens for (`->`)[Arrow::Single] and (`=>`)[Arrow::Double]
#[derive(PartialEq, Clone)]
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum Arrow {
	/// Single stem arrow `->`
	Single,
	/// Double stem arrow `=>`
	Double
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
			6 => PreTokType::ControlKeyword(td.into()),
			7 => PreTokType::DataKeyword(td.into()),
			8 => PreTokType::PrimitiveKeyword(td.into()),
			9 => PreTokType::Identifier(l.name.1.to_string(), matcher),
			10 => match td {
				0 => PreTokType::NewLine(NewLine::Implicit),
				1 => PreTokType::NewLine(NewLine::Explicit),
				_ => unreachable!(),
			}
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
			12 => PreTokType::Comma,
			13 => PreTokType::At,
			14 => PreTokType::Arrow(Arrow::Single),
			15 => PreTokType::Arrow(Arrow::Double),
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
