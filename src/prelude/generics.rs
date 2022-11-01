use std::fmt::Debug;
use std::path::PathBuf;

/// Error construction macro
#[macro_export]
macro_rules! error {
    ($e:literal) => {Err(32768+$e)};
	($e:literal, $($x:expr),+) => {Err(32768+$e)};
}

/// Warning construction macro
#[macro_export]
macro_rules! warn {
    ($e:literal) => {Err($e)};
	($e:literal, $($x:expr),+) => {Err($e)};
}

#[derive(Clone)]
pub enum TransitionMap<T: PartialEq> {
	Const(fn(T) -> u8),
	Generate(Vec<T>)
}

impl<T: PartialEq> From<Vec<T>> for TransitionMap<T> {
	fn from(v: Vec<T>) -> Self {
		TransitionMap::Generate(v)
	}
}

impl<T: PartialEq> From<fn(T) -> u8> for TransitionMap<T> {
	fn from(v: fn(T) -> u8) -> Self {
		TransitionMap::Const(v)
	}
}

impl<T: PartialEq> TransitionMap<T> {
	pub fn run(&self, c: T) -> u8 {
		match self {
			TransitionMap::Const(f) => f(c),
			TransitionMap::Generate(v) => v.iter().position(|x| x == &c).unwrap_or(255) as u8
		}
	}
}

impl<T: PartialEq + Debug> TransitionMap<T> {
	pub(crate) fn to_string(&self) -> String {
		match self {
			TransitionMap::Const(_) => {
				String::new()
			}
			TransitionMap::Generate(v) => {
				v.iter().enumerate().fold("|c: char| -> u8 { match c {".to_string(), |acc, (i, l)| {
					format!("{}{:?} => {},", acc, l, i)
				}) + "_ => 255 } }"
			}
		}
	}
}

pub enum Command {
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
