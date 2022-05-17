//! Language handling
//!
//! This contains all the keywords currently available for fck, as well as the functions that allow
//! the extended use of them. This crate (in the fck repo) also includes the `fckl equivalents`
//! folder that has the `.fckl` language files equivalent to all the currently included languages
use std::path::PathBuf;
use clap::{Arg, Command as Cmd};

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

/// Returns (assuming the language code is valid) the messages relating to the given language code
pub fn get_cli_keywords(lang_code: &str) -> Option<keywords::CLIKeywords<'static>> {
    match lang_code {
        "en" => Some(en::CLI_KEYWORDS),
        "de" => Some(de::CLI_KEYWORDS),
        "fr" => Some(fr::CLI_KEYWORDS),
        "ko" => Some(ko::CLI_KEYWORDS),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

/// # Custom format macro
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

pub enum  Command {
	New {
		path: PathBuf,
		git: bool
	},
	Shell,
	Build {
		path: PathBuf,
		llvm: bool
	},
	Run {
		path: PathBuf,
		llvm: bool,
		no_build: bool
	},
	Test {
		path: PathBuf,
		llvm: bool,
		tests: Vec<String>
	},
	Info,
	Lint {
		path: PathBuf
	},
	Raw {
		raw: String,
		llvm: bool,
	},
	Doc {
		path: PathBuf,
		no_build: bool
	},
	Translate {
		path: PathBuf,
		output: PathBuf,
		target_language: String,
		comment: bool
	}
}

pub fn get_cli(lang_code: &str) -> Option<Command> {
	let kwds = match lang_code {
		"en" => en::CLI_KEYWORDS,
		"de" => de::CLI_KEYWORDS,
		"fr" => fr::CLI_KEYWORDS,
		"ko" => ko::CLI_KEYWORDS,
		_ => return None
	};
	let args = [
		Arg::new(kwds.args[0].clone().0).help(kwds.args[0].clone().2).default_value(".").index(1),
		kwds.args[1].clone().clap(),
		kwds.args[2].clone().clap().takes_value(false),
		kwds.args[3].clone().clap().takes_value(false),
		kwds.args[4].clone().clap().multiple_occurrences(true).takes_value(true),
		Arg::new(kwds.args[5].clone().0).help(kwds.args[5].clone().2).default_value(".").required(true),
		kwds.args[6].clone().clap().required(true),
		kwds.args[7].clone().clap().takes_value(true),
		kwds.args[8].clone().clap().required(false)
	];
	let app = Cmd::new("fck")
		.about(kwds.desc)
		.subcommands([
			// new
			Cmd::new(kwds.commands[0].0.clone())
				.about(kwds.commands[0].1.clone())
				.arg(args[0].clone())
				.arg(args[1].clone()),
			// shell
			Cmd::new(kwds.commands[1].0.clone())
				.about(kwds.commands[1].1.clone()),
			// build
			Cmd::new(kwds.commands[2].0.clone())
				.about(kwds.commands[2].1.clone())
				.arg(args[0].clone())
				.arg(args[2].clone()),
			// run
			Cmd::new(kwds.commands[3].0.clone())
				.about(kwds.commands[3].1.clone())
				.arg(args[0].clone())
				.arg(args[2].clone())
				.arg(args[3].clone()),
			// test
			Cmd::new(kwds.commands[4].0.clone())
				.about(kwds.commands[4].1.clone())
				.arg(args[0].clone())
				.arg(args[2].clone())
				.arg(args[4].clone()),
			// info
			Cmd::new(kwds.commands[5].0.clone())
				.about(kwds.commands[5].1.clone()),
			// lint
			Cmd::new(kwds.commands[6].0.clone())
				.about(kwds.commands[6].1.clone())
				.arg(args[0].clone()),
			// raw
			Cmd::new(kwds.commands[7].0.clone())
				.about(kwds.commands[7].1.clone())
				.arg(args[5].clone())
				.arg(args[2].clone()),
			// doc
			Cmd::new(kwds.commands[8].0.clone())
				.about(kwds.commands[8].1.clone())
				.arg(args[0].clone())
				.arg(args[3].clone()),
			// translate
			Cmd::new(kwds.commands[9].0.clone())
				.about(kwds.commands[9].1.clone())
				.arg(args[0].clone())
				.arg(args[6].clone())
				.arg(args[7].clone())
				.arg(args[8].clone())
		]);
	Some(if let Some((c, t)) = app.get_matches().subcommand() {
		match kwds.commands.iter().map(|(d, _)| *d).collect::<Vec<&str>>().iter().position(|&d| d == c).unwrap() {
			0 => Command::New {
					path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
					git: t.is_present(kwds.args[1].0)
				},
			1 => Command::Shell,
			2 => Command::Build {
					path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
					llvm: t.is_present(kwds.args[2].0)
				},
			3 => Command::Run {
					path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
					llvm: t.is_present(kwds.args[2].0),
					no_build: t.is_present(kwds.args[3].0)
				},
			4 => Command::Test {
					path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
					llvm: t.is_present(kwds.args[2].0),
					tests: match t.values_of(kwds.args[4].0) {
						None => vec![],
						Some(vals) => vals.map(|val| val.to_string()).collect()
					}
				},
			5 => Command::Info,
			6 => Command::Lint { path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()) },
			7 => Command::Raw {
					raw: t.value_of(kwds.args[5].0).unwrap().to_string(),
					llvm: t.is_present(kwds.args[2].0)
				},
			8 => Command::Doc {
					path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
					no_build: t.is_present(kwds.args[3].0)
				},
			9 => Command::Translate {
				path: PathBuf::from(t.value_of(kwds.args[0].0).unwrap()),
				output: if let Some(p) = t.value_of(kwds.args[7].0) {
					PathBuf::from(p)
				} else {
					let mut out = PathBuf::from(t.value_of(kwds.args[0].0).unwrap());
					out.set_file_name(format!("{}_{}.fck", out.file_stem().unwrap().to_str().unwrap(), t.value_of(kwds.args[6].0).unwrap()));
					out
				},
				target_language: t.value_of(kwds.args[6].0).unwrap().to_string(),
				comment: t.is_present(kwds.args[7].0)
			},
			_ => unreachable!()
		}
	} else {
		Command::Shell
	})
}
