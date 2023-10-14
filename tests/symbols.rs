use lang::tok::{Op, Token, TokType, Cmp, Position, Arrow, NewLine};
use lang_inner::{LanguageRaw, Table};

#[test]
fn symbols() {
	let buf = Vec::new();
	let (l, m): (&LanguageRaw, (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>)) = lang::get("en", &buf).unwrap();
	println!("{} {} {}", m.0.element(0, 10), m.1.element(0, 10), m.2.element(0, 10));
	for (i, e) in [
		("->", TokType::Arrow(Arrow::Single)),
		("=>", TokType::Arrow(Arrow::Double)),
		("@", TokType::At),
		(",", TokType::Comma),
		(".", TokType::Dot),
		(":", TokType::Colon),
		(";", TokType::NewLine(NewLine::Explicit)),
		("\n", TokType::NewLine(NewLine::Implicit)),
	] {
		match lang::tokenize(i.bytes(), l, &buf, m) {
			Ok(toks) => {
				println!("Input: {:?}", i);
				if toks.len() != 1 {
					println!("Incorrect number of tokens returned");
					println!("Returned token(s) ({}):", toks.len());
					for i in toks.iter() { println!("- {:?}", i); }
					println!("Expected token: {:?}", e);
					panic!()
				}
				println!("Returned token: {:?}", toks.first().unwrap());
				println!("Expected token: {:?}", e);
				let tok = toks.into_iter().next().unwrap();
				assert_eq!(tok, e);
				println!();
			}
			Err(err) => assert!(false, "Failed parsing {:?}: {}", i, err)
		}
	}
}
