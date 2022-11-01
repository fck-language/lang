macro_rules! inc {
    ($($m:ident),+) => {
		fn main() -> Result<(), std::io::Error> {
			$(std::fs::write(concat!("src/generated/", stringify!($m), ".rs"), "");)+
			std::fs::write(
				"src/languages/mod.rs",
				concat!(
					$( concat!("pub mod ", stringify!($m), ";") ),+
				)
			)?;
			std::fs::write("src/main.rs", concat!(
				"use std::io::Write;mod prelude;mod languages;\n",
				stringify!(fn main() -> Result<(), std::io::Error>{
					std::fs::write("src/generated/mod.rs", "")?;
					let mut a = std::fs::OpenOptions::new().write(true).append(true).open("src/generated/mod.rs")?;
					$(
						let (map, acc, c_map) = languages::$m::LANG.kwd_to_maps().unwrap();
						std::fs::write(
							concat!("src/generated/", stringify!($m), ".rs"),
							format!("use crate::prelude::{{TokType::*, Language, TransitionMap}};\npub const LANG: Language = Language {{ transition_map: &{:?}, accepting_map: &{:?}, raw_language: &crate::languages::{}::LANG, mapping: TransitionMap::Const({}) }};", map, acc, stringify!($m), c_map.to_string())
						)?;
						write!(a, concat!("pub mod ", stringify!($m), ";"))?;
					)+
					std::fs::write("src/lib.rs", concat!(
						"pub mod prelude;pub mod languages;pub mod generated;\n",
						stringify!(pub fn get<'a>(c: &str) -> Result<prelude::Language<'a>, ()> { match c { $(concat!("\"", stringify!($m), "\"") => Ok(generated::$m::LANG),)+_ => Err(())}})
					))?;
					Ok(())
				})
			))?;
			Ok(())
		}
	};
}

inc!(en, de);
