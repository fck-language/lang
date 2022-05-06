//! Language handling
//!
//! This contains all the keywords currently available for fck, as well as the functions that allow
//! the extended use of them. This crate (in the fck repo) also includes the `fckl equivalents`
//! folder that has the `.fckl` language files equivalent to all the currently included languages
use clap::{Arg, Command};

pub mod keywords;
pub mod en;
pub mod fr;
pub mod de;
pub mod ko;

/// Returns (assuming the language code is valid) the keywords relating to the given language code
pub fn get_associated_keywords(lang_code: &str) -> Option<keywords::Keywords<'static>> {
    match lang_code {
        "en" => Some(en::KEYWORDS),
        "de" => Some(de::KEYWORDS),
        "fr" => Some(fr::KEYWORDS),
        "ko" => Some(ko::KEYWORDS),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

/// Returns (assuming the language code is valid) the messages relating to the given language code
pub fn get_associated_messages(lang_code: &str) -> Option<keywords::Messages<'static>> {
    match lang_code {
        "en" => Some(en::MESSAGES),
        "de" => Some(de::MESSAGES),
        "fr" => Some(fr::MESSAGES),
        "ko" => Some(ko::MESSAGES),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

/// Custom format macro
///
/// Used similarly to the normal `format!` macro, with some alterations. The first variable can be
/// a `&str` or `String` and can be a variable. Any `_` is replaced with the index appropriate
/// value. All formatting is the `std::fmt::Display` not `std::fmt::Debug` so beware. this is
/// required to be able to use language specific string formats that can be formatted such as error
/// messages with information embedded in them. **DO NOT CALL THIS WITH ONLY A FORMAT STRING**
#[macro_export]
macro_rules! fmt {
    ($f: expr, $($a: expr), *) => {{
        let mut out = String::new();
		let F = format!(" {} ", $f);
		let mut iter = F.split('_');
		$(
		out = format!("{}{}{}", out, iter.next().unwrap(), $a);
		)*
		out = format!("{}{}", out, iter.next().unwrap());
        out.get(1..out.len() - 1).unwrap().to_string()
    }}
}

pub fn get_cli(lang_code: &str) -> Option<Command> {
	let kwds = match lang_code {
		"en" => en::CLI_KEYWORDS,
		"de" => de::CLI_KEYWORDS,
		"fr" => fr::CLI_KEYWORDS,
		"ko" => ko::CLI_KEYWORDS,
		_ => return None
	};
	let debug_dump = [
		Arg::new("debug").long(kwds.single_flag_args[1].0.clone()).help(kwds.single_flag_args[1].1.clone()).takes_value(false),
		Arg::new("llvm").long(kwds.single_flag_args[2].0.clone()).help(kwds.single_flag_args[2].1.clone()).takes_value(false)
	];
	Some(
		Command::new("fck")
			.subcommands([
				// new
				Command::new(kwds.commands[0].0.clone())
					.about(kwds.commands[0].1.clone())
					.arg(Arg::new("directory").help(kwds.help_strings[0].clone()).index(0).required(true))
					.arg(Arg::new("git").long(kwds.single_flag_args[0].0.clone()).help(kwds.single_flag_args[0].1.clone())),
				// shell
				Command::new(kwds.commands[1].0.clone())
					.about(kwds.commands[1].1.clone()),
				// build
				Command::new(kwds.commands[2].0.clone())
					.about(kwds.commands[2].1.clone())
					.arg(Arg::new("path").help(kwds.help_strings[1].clone()))
					.arg(Arg::new("debug").long(kwds.single_flag_args[0].0.clone()).takes_value(false))
					.arg(Arg::new("llvm").long(kwds.single_flag_args[1].0.clone()).takes_value(false)),
				// run
				Command::new(kwds.commands[3].0.clone())
					.about(kwds.commands[3].1.clone())
					.arg(Arg::new("path").help(kwds.help_strings[1].clone()))
					.args(debug_dump.clone())
					.arg(Arg::new("no build").long(kwds.single_flag_args[3].0.clone()).help(kwds.single_flag_args[3].1.clone()).takes_value(false)),
				// test
				Command::new(kwds.commands[4].0.clone())
					.about(kwds.commands[4].1.clone())
					.arg(Arg::new("path").help(kwds.help_strings[1].clone()))
					.args(debug_dump.clone())
					.arg(Arg::new("test").short(kwds.double_flag_args[0].0.clone()).long(kwds.double_flag_args[0].1.clone()).help(kwds.double_flag_args[0].2.clone()).multiple_occurrences(true).takes_value(true)),
				// info
				Command::new(kwds.commands[5].0.clone())
					.about(kwds.commands[5].1.clone()),
				// lint
				Command::new(kwds.commands[6].0.clone())
					.about(kwds.commands[6].1.clone())
					.arg(Arg::new("path").help(kwds.help_strings[1].clone())),
				// raw
				Command::new(kwds.commands[7].0.clone())
					.about(kwds.commands[7].1.clone())
					.arg(Arg::new("raw").help(kwds.help_strings[2].clone()).required(true))
					.args(debug_dump.clone()),
				// doc
				Command::new(kwds.commands[8].0.clone())
					.about(kwds.commands[8].1.clone())
					.arg(Arg::new("path").help(kwds.help_strings[1].clone()))
					.arg(Arg::new("no build").long(kwds.single_flag_args[3].0.clone()).help(kwds.single_flag_args[3].1.clone()).takes_value(false))
			])
	)
}
