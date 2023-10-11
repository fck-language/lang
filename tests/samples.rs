use lang::tok::{Token, Position, TokType, ControlKeyword, DataKeyword, PrimitiveKeyword, Arrow, Op, Cmp};
use num_bigint::BigUint;

macro_rules! wrap_pos {
    (($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $ty:expr) => { Token {
		ps: Position { ln: $ps1, col: $ps2 },
		pe: Position { ln: $pe1, col: $pe2 },
		tt: $ty
	} };
}

macro_rules! value {
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $ty:tt) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), value!($ty)) };
    [true] => { TokType::Bool(true) };
    [false] => { TokType::Bool(false) };
    [+] => { TokType::Op(Op::Plus) };
    [-] => { TokType::Op(Op::Minus) };
    [%] => { TokType::Op(Op::Mod) };
    [*] => { TokType::Op(Op::Mult) };
    [/] => { TokType::Op(Op::Div) };
    [**] => { TokType::Op(Op::Pow) };
    [==] => { TokType::Cmp(Cmp::Eq) };
    [<] => { TokType::Cmp(Cmp::LT) };
    [<=] => { TokType::Cmp(Cmp::LTE) };
    [>] => { TokType::Cmp(Cmp::GT) };
    [>=] => { TokType::Cmp(Cmp::GTE) };
    [!=] => { TokType::Cmp(Cmp::NE) };
    [++] => { TokType::Increment };
    [--] => { TokType::Decrement };
    [!] => { TokType::Not };
    [:] => { TokType::Colon };
    [.] => { TokType::Dot };
    [?] => { TokType::QuestionMark };
    [->] => { TokType::Arrow(Arrow::Single) };
    [=>] => { TokType::Arrow(Arrow::Double) };
	[=] => { TokType::Set(None) };
    [+=] => { TokType::Set(Some(Op::Plus)) };
    [-=] => { TokType::Set(Some(Op::Minus)) };
    [%=] => { TokType::Set(Some(Op::Mod)) };
    [*=] => { TokType::Set(Some(Op::Mult)) };
    [/=] => { TokType::Set(Some(Op::Div)) };
    [**=] => { TokType::Set(Some(Op::Pow)) };
}

macro_rules! int {
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $value:literal) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), int!($value)) };
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $value:literal, $base:literal) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), int!($value, $base)) };
    ($value:literal) => { int!($value, 10) };
    ($value:literal, $base:literal) => { TokType::Int(BigUint::parse_bytes(stringify!($value).as_bytes(), $base).unwrap()) };
}

macro_rules! kwd {
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $ty:tt) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), kwd!($ty)) };
	(Set) => { TokType::ControlKeyword(ControlKeyword::KSet) };
	(And) => { TokType::ControlKeyword(ControlKeyword::KAnd) };
	(Or) => { TokType::ControlKeyword(ControlKeyword::KOr) };
	(Not) => { TokType::ControlKeyword(ControlKeyword::KNot) };
	(If) => { TokType::ControlKeyword(ControlKeyword::KIf) };
	(Else) => { TokType::ControlKeyword(ControlKeyword::KElse) };
	(Match) => { TokType::ControlKeyword(ControlKeyword::KMatch) };
	(Repeat) => { TokType::ControlKeyword(ControlKeyword::KRepeat) };
	(For) => { TokType::ControlKeyword(ControlKeyword::KFor) };
	(In) => { TokType::ControlKeyword(ControlKeyword::KIn) };
	(To) => { TokType::ControlKeyword(ControlKeyword::KTo) };
	(As) => { TokType::ControlKeyword(ControlKeyword::KAs) };
	(While) => { TokType::ControlKeyword(ControlKeyword::KWhile) };
	(Fn) => { TokType::ControlKeyword(ControlKeyword::KFn) };
	(Return) => { TokType::ControlKeyword(ControlKeyword::KReturn) };
	(Continue) => { TokType::ControlKeyword(ControlKeyword::KContinue) };
	(Break) => { TokType::ControlKeyword(ControlKeyword::KBreak) };
	(Where) => { TokType::ControlKeyword(ControlKeyword::KWhere) };
	(Struct) => { TokType::DataKeyword(DataKeyword::KStruct) };
	(Properties) => { TokType::DataKeyword(DataKeyword::KProperties) };
	(Enum) => { TokType::DataKeyword(DataKeyword::KEnum) };
	(Variants) => { TokType::DataKeyword(DataKeyword::KVariants) };
	(_Self) => { TokType::DataKeyword(DataKeyword::KSelf) };
	(SSelf) => { TokType::DataKeyword(DataKeyword::KSSelf) };
	(Extension) => { TokType::DataKeyword(DataKeyword::KExtension) };
	(Extend) => { TokType::DataKeyword(DataKeyword::KExtend) };
	(Int) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KInt) };
	(Uint) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KUint) };
	(Dint) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KDint) };
	(Udint) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KUdint) };
	(Float) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KFloat) };
	(Bfloat) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KBfloat) };
	(Str) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KStr) };
	(Char) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KChar) };
	(List) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KList) };
	(Bool) => { TokType::PrimitiveKeyword(PrimitiveKeyword::KBool) };
}

macro_rules! ident {
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $ident:ident) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), ident!($ident)) };
	(($ps1:expr, $ps2:expr), ($pe1:expr, $pe2:expr), $lang:ident : $ident:ident) => { wrap_pos!(($ps1, $ps2), ($pe1, $pe2), ident!($lang : $ident)) };
    ($ident:ident) => { TokType::Identifier("en".to_string(), stringify!($ident).as_bytes().to_vec()) };
    ($lang:ident : $ident:ident) => { TokType::Identifier(stringify!($lang).to_string(), stringify!($ident).as_bytes().to_vec()) };
}

macro_rules! test_input {
    ($path:ident, $($expected:expr),*$(,)?) => {
		#[test]
		fn $path() -> Result<(), u16> {
			let input = include_str!(concat!("sample scripts/", stringify!($path), ".fck"));
			let expected = vec![$($expected),*];
			let b = input.bytes();
			let buf = Vec::new();
			let (l, m) = lang::get("en", &buf).unwrap();
			match lang::tokenize(b, l, &buf, m) {
				Ok(toks) => {
					if toks.len() != expected.len() {
						println!("Incorrect number of tokens returned");
						println!("Returned tokens ({}):", toks.len());
						for i in toks.iter() { println!("- {:?}", i); }
						println!("Expected tokens ({}):", expected.len());
						for i in expected.iter() { println!("- {:?}", i); }
						panic!()
					}
					println!("Returned tokens:");
					for i in toks.iter() { println!("- {:?}", i); }
					println!("Expected tokens:");
					for i in expected.iter() { println!("- {:?}", i); }
					println!("\n{:^70}    {:^70}", "Returned", "Expected");
					for (i, l) in toks.iter().zip(expected) {
						println!("{:<70} {}= {:?}", format!("{:?}", *i), if *i == l { '=' } else { '!' }, l);
						assert_eq!(*i, l)
					}
					Ok(())
				}
				Err(e) => Err(e)
			}
		}
	};
}

test_input!(
	test1,
	kwd!((1, 0), (1, 3), Set), ident!((1, 4), (1, 10), my_var), value!((1, 11), (1, 12), =), int!((1, 13), (1, 14), 5),
	kwd!((2, 0), (2, 6), Struct), ident!((2, 7), (2, 17), SomeStruct), wrap_pos!((2, 18), (2, 19), TokType::LParenCurly),
		kwd!((3, 4), (3, 14), Properties), wrap_pos!((3, 15), (3, 16), TokType::LParenCurly),
			kwd!((3, 17), (3, 20), Int), ident!((3, 21), (3, 26), inner),
		wrap_pos!((3, 27), (3, 28), TokType::RParenCurly),
		wrap_pos!((4, 4), (4, 5), TokType::LParenCurly),
			kwd!((5, 8), (5, 10), Fn), ident!((5, 11), (5, 17), de:set_de), wrap_pos!((5, 17), (5, 18), TokType::LParen), wrap_pos!((5, 18), (5, 19), TokType::RParen), wrap_pos!((5, 20), (5, 21), TokType::LParenCurly), wrap_pos!((5, 22), (5, 23), TokType::RParenCurly),
		wrap_pos!((6, 4), (6, 5), TokType::RParenCurly),
		kwd!((7, 4), (7, 6), Fn), ident!((7, 7), (7, 11), set_), wrap_pos!((7, 11), (7, 12), TokType::LParen), kwd!((7, 12), (7, 15), Int), ident!((7, 16), (7, 21), inner), wrap_pos!((7, 21), (7, 22), TokType::RParen), value!((7, 23), (7, 25), ->), kwd!((7, 26), (7, 30), SSelf), wrap_pos!((7, 31), (7, 32), TokType::LParenCurly),
			kwd!((8, 8), (8, 14), Return), kwd!((8, 15), (8, 19), SSelf), wrap_pos!((8, 20), (8, 21), TokType::LParenCurly), ident!((8, 22), (8, 27), inner), value!((8, 27), (8, 28), :), ident!((8, 29), (8, 34), inner), wrap_pos!((8, 35), (8, 36), TokType::RParenCurly),
		wrap_pos!((9, 4), (9, 5), TokType::RParenCurly),
	wrap_pos!((10, 0), (10, 1), TokType::RParenCurly)
);

test_input!(
	test2,
	kwd!(Set), TokType::LParenCurly, kwd!(Set), kwd!(Set), TokType::RParenCurly, kwd!(Set), kwd!(Set)
);

test_input!(
	test3,
	int!((1, 0), (1, 10), 0123456789),
	int!((2, 0), (2, 4), 01, 2),
	int!((3, 0), (3, 10), 01234567, 8),
	int!((4, 0), (4, 24), 0123456789abcdefABCDEF, 16)
);
