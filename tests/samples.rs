use lang::tok::{TokType, Op, Cmp};
use num_bigint::BigUint;

macro_rules! value {
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
	[=] => { TokType::Set(None) };
    [+=] => { TokType::Set(Some(Op::Plus)) };
    [-=] => { TokType::Set(Some(Op::Minus)) };
    [%=] => { TokType::Set(Some(Op::Mod)) };
    [*=] => { TokType::Set(Some(Op::Mult)) };
    [/=] => { TokType::Set(Some(Op::Div)) };
    [**=] => { TokType::Set(Some(Op::Pow)) };
}

macro_rules! int {
    ($value:literal) => { int!($value, 10) };
    ($value:literal, $base:literal) => { TokType::Int(BigUint::parse_bytes(stringify!($value).as_bytes(), $base).unwrap()) };
}

macro_rules! kwd {
    ($val:literal) => { TokType::Keyword($val) };
	(set) => { TokType::Keyword(0) };
	(and) => { TokType::Keyword(1) };
	(or) => { TokType::Keyword(2) };
	(not) => { TokType::Keyword(3) };
	(_if) => { TokType::Keyword(4) };
	(_else) => { TokType::Keyword(5) };
	(case) => { TokType::Keyword(6) };
	(default) => { TokType::Keyword(7) };
	(iterate) => { TokType::Keyword(8) };
	(import) => { TokType::Keyword(9) };
	(_while) => { TokType::Keyword(10) };
	(def) => { TokType::Keyword(11) };
	(_return) => { TokType::Keyword(12) };
	(_continue) => { TokType::Keyword(13) };
	(_break) => { TokType::Keyword(14) };
	(_as) => { TokType::Keyword(15) };
	(class) => { TokType::Keyword((1 << 6) + 0) };
	(properties) => { TokType::Keyword((1 << 6) + 1) };
	(_enum) => { TokType::Keyword((1 << 6) + 2) };
	(variants) => { TokType::Keyword((1 << 6) + 3) };
	(_self) => { TokType::Keyword((1 << 6) + 4) };
	(_Self) => { TokType::Keyword((1 << 6) + 5) };
	(extension) => { TokType::Keyword((1 << 6) + 6) };
	(extend) => { TokType::Keyword((1 << 6) + 7) };
	(int) => { TokType::Keyword((2 << 6) + 0) };
	(float) => { TokType::Keyword((2 << 6) + 1) };
	(bool) => { TokType::Keyword((2 << 6) + 2) };
	(str) => { TokType::Keyword((2 << 6) + 3) };
	(list) => { TokType::Keyword((2 << 6) + 4) };
	(map) => { TokType::Keyword((2 << 6) + 5) };
}

macro_rules! ident {
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
					for i in toks.iter() { println!("{:?}", i); }
					assert_eq!(toks.len(), expected.len(), "Incorrect number of tokens returned");
					for (i, l) in toks.iter().zip(expected) {
						assert_eq!(i.tt, l)
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
	kwd!(set), ident!(my_var), value!(=), int!(5),
	kwd!(class), ident!(SomeClass), TokType::LParenCurly,
		kwd!(properties), TokType::LParenCurly, kwd!(int), ident!(inner), TokType::RParenCurly,
		TokType::LParenCurly,
			kwd!(def), ident!(de:set_de), TokType::LParen, TokType::RParen, TokType::LParenCurly, TokType::RParenCurly,
		TokType::RParenCurly,
		kwd!(def), ident!(set_), TokType::LParen, kwd!(int), ident!(inner), TokType::RParen, value!(-), value!(>), kwd!(_Self), TokType::LParenCurly,
			kwd!(_return), kwd!(_Self), TokType::LParenCurly, ident!(inner), value!(:), ident!(inner), TokType::RParenCurly,
		TokType::RParenCurly,
	TokType::RParenCurly
);

test_input!(
	test2,
	kwd!(0), TokType::LParenCurly, kwd!(0), kwd!(0), TokType::RParenCurly, kwd!(0), kwd!(0)
);

test_input!(
	test3,
	int!(0123456789), int!(01, 2), int!(01234567, 8), int!(0123456789abcdefABCDEF, 16),
	int!(0123456789), int!(01, 2), int!(01234567, 8), int!(0123456789abcdef, 16)
);
