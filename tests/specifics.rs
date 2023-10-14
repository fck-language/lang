use lang::tok::{Op, Token, TokType, Cmp, Position};
use lang_inner::{LanguageRaw, Table};
use std::ops::Deref;

#[cfg(test)]
mod operators {
	use lang::tok::NewLine;
	use super::*;
	
	macro_rules! operators {
		($(($name:ident, $input:literal, $out:expr)),*$(,)?) => { $(operators!{@inner, $name, $input, $out})* };
		(@inner, $name:ident, $input:literal, $out:expr) => {
			#[test]
			fn $name() {
				let buf = Vec::new();
				let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
				let s = $input.to_string();
				
				match lang::tokenize(s.bytes(), l, &buf, m) {
					Ok(mut res) => {
						assert_eq!(res.len(), 1, "Returned wrong number of tokens");
						assert_eq!(res.pop().unwrap().tt, $out, "Incorrect token returned")
					}
					Err(err) => assert!(false, "Failed parsing: {}", err)
				}
			}
		};
	}
	
	#[test]
	fn space_newline() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "\n;\n";
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(mut res) => {
				assert_eq!(res.len(), 3, "Returned wrong number of tokens");
				assert_eq!(res, vec![
					Token { ps: Position { ln: 0, col: 0 }, pe: Position { ln: 1, col: 0 }, tt: TokType::NewLine(NewLine::Implicit) },
					Token { ps: Position { ln: 1, col: 0 }, pe: Position { ln: 1, col: 1 }, tt: TokType::NewLine(NewLine::Explicit) },
					Token { ps: Position { ln: 1, col: 1 }, pe: Position { ln: 2, col: 0 }, tt: TokType::NewLine(NewLine::Implicit) }
				])
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn rparen_curl() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "}".to_string();
		
		assert!(lang::tokenize(s.bytes(), l, &buf, m).is_err(), "Input should fail");
	}
	
	operators!(
		(plus, "+", TokType::Op(Op::Plus)), (inc, "++", TokType::Increment), (plus_set, "+=", TokType::Set(Some(Op::Plus))),
		(sub, "-", TokType::Op(Op::Minus)), (dec, "--", TokType::Decrement), (sub_set, "-=", TokType::Set(Some(Op::Minus))),
		(mult, "*", TokType::Op(Op::Mult)), (pow, "**", TokType::Op(Op::Pow)), (mult_set, "*=", TokType::Set(Some(Op::Mult))), (pow_set, "**=", TokType::Set(Some(Op::Pow))),
		(div, "/", TokType::Op(Op::Div)), (div_set, "/=", TokType::Set(Some(Op::Div))), (modulus, "%", TokType::Op(Op::Mod)), (mod_set, "%=", TokType::Set(Some(Op::Mod))),
		(not, "!", TokType::Not), (not_eq, "!=", TokType::Cmp(Cmp::NE)), (set, "=", TokType::Set(None)), (eq, "==", TokType::Cmp(Cmp::Eq)),
		(lt, "<", TokType::Cmp(Cmp::LT)), (gt, ">", TokType::Cmp(Cmp::GT)), (lte, "<=", TokType::Cmp(Cmp::LTE)), (gte, ">=", TokType::Cmp(Cmp::GTE)),
		(colon, ":", TokType::Colon), (question, "?", TokType::QuestionMark), (dot, ".", TokType::Dot),
		(lparen, "(", TokType::LParen), (rparen, ")", TokType::RParen), (lparen_curl, "{", TokType::LParenCurly), (lparen_square, "[", TokType::LParenSquare), (rparen_square, "]", TokType::RParenSquare)
	);
}

#[cfg(test)]
mod digits {
	use super::*;
	
	macro_rules! digit_test {
	    ($name:ident, $test:literal, $($v:literal),*$(,)?) => {
			#[test]
			fn $name() -> Result<(), u16> {
				let test = $test;
				let expected = vec![$(TokType::Int(($v as u16).into())),*];
				let buf = Vec::new();
				let (l, (transition, tt, td)) = lang::get("en", &buf).unwrap();
				match lang::tokenize(
					test.bytes(),
					l, &buf, (transition, tt, td),
				) {
					Ok(toks) => {
						for t in toks.iter() { println!("{:?}", t) }
						assert_eq!(toks.len(), expected.len(), "Incorrect number of tokens returned");
						for (tok, tt) in toks.iter().zip(expected.iter()) {
							assert_eq!(tok, tt)
						}
						Ok(())
					}
					Err(e) => Err(e),
				}
			}
		};
	}
	
	digit_test!(ints_b10, "0 1 2 3 4 5 6 7 8 9 513 0839", 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 513, 839);
	digit_test!(ints_b2, "0b0 0b1 0b1011 0b01", 0, 1, 0b1011, 0b1);
	digit_test!(ints_b8, "0o0 0o1 0o2 0o3 0o4 0o5 0o6 0o7 0o1357 0o0162", 0, 1, 2, 3, 4, 5, 6, 7, 0o1357, 0o162);
	digit_test!(ints_b16,
		"0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7 0x8 0x9 0xa 0xb 0xc 0xd 0xe 0xf 0xA 0xB 0xC 0xD 0xE 0xF 0xA0b 0x0f3E",
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 10, 11, 12, 13, 14, 15, 0xA0b, 0x0f3E
	);
}

#[cfg(test)]
mod comments {
	use lang::tok::NewLine;
	use super::*;
	
	#[test]
	fn full_line() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = r"\\ some comment".to_string() + "\n" + r"123";
		let expected = vec![
			TokType::Comment("en".to_string(), " some comment".as_bytes().to_vec()),
			TokType::Int(123u16.into())
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn block() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = r"\*no spaces*\ \* spaces *\".to_string() + "\n" + r"123 \* inline *\ 456" + "\n" + r"\* over several" + "\n" + r"lines *\ +";
		println!("{}", s);
		let expected = vec![
			TokType::Comment("en".to_string(), "no spaces".as_bytes().to_vec()),
			TokType::Comment("en".to_string(), " spaces ".as_bytes().to_vec()),
			TokType::NewLine(NewLine::Implicit),
			TokType::Int(123u16.into()),
			TokType::Comment("en".to_string(), " inline ".as_bytes().to_vec()),
			TokType::Int(456u16.into()),
			TokType::NewLine(NewLine::Implicit),
			TokType::Comment("en".to_string(), " over several\nlines ".as_bytes().to_vec()),
			TokType::Op(Op::Plus)
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
}

#[cfg(test)]
mod idents {
	use lang::tok::ControlKeyword;
	use super::*;
	
	#[test]
	fn ident_s_kwd() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "ident else".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
			TokType::ControlKeyword(ControlKeyword::KElse),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn digits() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "0 ident i0 0i".to_string();
		let expected = vec![
			TokType::Int(0u16.into()),
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
			TokType::Identifier("en".to_string(), "i0".as_bytes().to_vec()),
			TokType::Int(0u16.into()),
			TokType::Identifier("en".to_string(), "i".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn kwd_extend() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "elsee".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "elsee".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn extend_kwd() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "eelse".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "eelse".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn ident_op_ident() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "ident+ident".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
			TokType::Op(Op::Plus),
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn ident_s_op_s_ident() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "ident + ident".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
			TokType::Op(Op::Plus),
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
	
	#[test]
	fn ident_op_s_ident() {
		let buf = Vec::new();
		let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
		let s = "ident+ ident".to_string();
		let expected = vec![
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
			TokType::Op(Op::Plus),
			TokType::Identifier("en".to_string(), "ident".as_bytes().to_vec()),
		];
		
		let buf = Vec::new();
		match lang::tokenize(s.bytes(), l, &buf, m) {
			Ok(res) => {
				for i in res.iter() { println!("{:?}", i) }
				assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
				for (l, r) in res.iter().zip(expected.iter()) {
					assert_eq!(l, r, "Incorrect token")
				}
			}
			Err(err) => assert!(false, "Failed parsing: {}", err)
		}
	}
}

#[cfg(test)]
mod string_char {
	use super::*;
	
	mod string {
		use super::*;
		
		#[test]
		fn string() {
			let buf = Vec::new();
			let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
			let s = "\"hello\"".to_string();
			let expected = vec![
				TokType::String("hello".as_bytes().to_vec()),
			];
			
			let buf = Vec::new();
			match lang::tokenize(s.bytes(), l, &buf, m) {
				Ok(res) => {
					for i in res.iter() { println!("{:?}", i) }
					assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
					for (l, r) in res.iter().zip(expected.iter()) {
						assert_eq!(l, r, "Incorrect token")
					}
				}
				Err(err) => assert!(false, "Failed parsing: {}", err)
			}
		}
		
		#[test]
		fn string_escape() {
			let buf = Vec::new();
			let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
			let s = "\"hello\\nworld\"".to_string();
			let expected = vec![
				TokType::String("hello\nworld".as_bytes().to_vec()),
			];
			
			let buf = Vec::new();
			match lang::tokenize(s.bytes(), l, &buf, m) {
				Ok(res) => {
					for i in res.iter() { println!("{:?}", i) }
					assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
					for (l, r) in res.iter().zip(expected.iter()) {
						assert_eq!(l, r, "Incorrect token")
					}
				}
				Err(err) => assert!(false, "Failed parsing: {}", err)
			}
		}
		
		#[test]
		fn string_newline() {
			let buf = Vec::new();
			let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
			let s = "\"hello\nme\"".to_string();
			let expected = vec![
				TokType::String("hello\nme".as_bytes().to_vec()),
			];
			
			let buf = Vec::new();
			match lang::tokenize(s.bytes(), l, &buf, m) {
				Ok(res) => {
					for i in res.iter() { println!("{:?}", i) }
					assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
					for (l, r) in res.iter().zip(expected.iter()) {
						assert_eq!(l, r, "Incorrect token")
					}
				}
				Err(err) => assert!(false, "Failed parsing: {}", err)
			}
		}
	}
	
	mod char {
		use super::*;
		
		#[test]
		fn char() {
			let buf = Vec::new();
			let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
			let s = "'e'".to_string();
			let expected = vec![
				TokType::Char('e'),
			];
			
			let buf = Vec::new();
			match lang::tokenize(s.bytes(), l, &buf, m) {
				Ok(res) => {
					for i in res.iter() { println!("{:?}", i) }
					assert_eq!(res.len(), expected.len(), "Returned wrong number of tokens");
					for (l, r) in res.iter().zip(expected.iter()) {
						assert_eq!(l, r, "Incorrect token")
					}
				}
				Err(err) => assert!(false, "Failed parsing: {}", err)
			}
		}
		
		#[test]
		fn double_char_fail() {
			let buf = Vec::new();
			let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
			let s = "'ee'".to_string();
			
			let buf = Vec::new();
			let res = lang::tokenize(s.bytes(), l, &buf, m);
			println!("{:?}", res);
			assert!(res.is_err(), "Should not be able to parse double character as single")
		}
	}
}
