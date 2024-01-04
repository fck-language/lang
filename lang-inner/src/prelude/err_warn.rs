//! Error and warning structs

use std::ops::Index;
use std::str::FromStr;
use crate::de::Deserialize;

/// # Error message and descriptors
///
/// Holds all the error messages and descriptors
#[derive(Copy, Clone)]
#[warn(missing_docs)]
pub struct Errors<'a> {
    /// Language based errors
    pub e00: [&'a str; 7],
    pub e01: [&'a str; 2],
    pub e02: [&'a str; 9],
    pub e03: [&'a str; 1],
    pub e04: [&'a str; 2],
}

impl<'a> Deserialize<'a> for Errors<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
		macro_rules! field {
			($name:ident, $l:literal) => {
				let $name = (0..$l)
					.map(|i| s.next().expect(&*format!("Expected {}{:>20}", stringify!($name), i)))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap();
			};
		}
		field!(e00, 7);
		field!(e01, 2);
		field!(e02, 9);
		field!(e03, 1);
		field!(e04, 2);
		Ok(Self { e00, e01, e02, e03, e04 })
    }
}

impl<'a> Index<(u8, u8)> for Errors<'a> {
    type Output = str;
    
    fn index(&self, (i1, i2): (u8, u8)) -> &Self::Output {
        match i1 {
            0 => self.e00[i2 as usize],
            1 => self.e01[i2 as usize],
            2 => self.e02[i2 as usize],
            3 => self.e03[i2 as usize],
            4 => self.e04[i2 as usize],
            _ => panic!("Out of bounds")
        }
    }
}

/// # Warning message and descriptors
///
/// Holds all the warning messages and descriptors
#[derive(Copy, Clone)]
#[warn(missing_docs)]
pub struct Warns<'a> {
    pub w00: [&'a str; 0],
    pub w01: [&'a str; 0],
    pub w02: [&'a str; 0],
    pub w03: [&'a str; 0],
    pub w04: [&'a str; 0],
}

impl<'a> Deserialize<'a> for Warns<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
		macro_rules! field {
			($name:ident, $l:literal) => {
				let $name = (0..$l)
					.map(|i| s.next().expect(&*format!("Expected {}{:>20}", stringify!($name), i)))
					.collect::<Vec<_>>()
					.try_into()
					.unwrap();
			};
		}
		field!(w00, 0);
		field!(w01, 0);
		field!(w02, 0);
		field!(w03, 0);
		field!(w04, 0);
		Ok(Self { w00, w01, w02, w03, w04 })
    }
}

impl<'a> Index<(u8, u8)> for Warns<'a> {
    type Output = str;
    
    fn index(&self, (i1, i2): (u8, u8)) -> &Self::Output {
        match i1 {
            0 => self.w00[i2 as usize],
            1 => self.w01[i2 as usize],
            2 => self.w02[i2 as usize],
            3 => self.w03[i2 as usize],
            4 => self.w04[i2 as usize],
            _ => panic!("Out of bounds")
        }
    }
}

/// Holds all the CLI commands, arguments, and help descriptions
#[derive(Copy, Clone)]
pub struct CLIKeywords<'a> {
    /// fck CLI description
    pub desc: &'a str,
    /// Commands and help descriptions
    pub commands: CLICommands<'a>,
    /// Single flag arguments with help messages
    pub args: CLIArgs<'a>,
}

impl<'a> Deserialize<'a> for CLIKeywords<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        Ok(Self {
            desc: s.next().expect("Expected CLI desc: &str"),
            commands: CLICommands::deserialize(s)?,
            args: CLIArgs::deserialize(s)?,
        })
    }
}

/// CLI commands. All are of the type (command word, help)
#[derive(Copy, Clone)]
pub struct CLICommands<'a> {
	/// new project
	pub k_new: (&'a str, &'a str),
	/// open a shell
	pub k_shell: (&'a str, &'a str),
	/// build project
	pub k_build: (&'a str, &'a str),
	/// run project
	pub k_run: (&'a str, &'a str),
	/// test project
	pub k_test: (&'a str, &'a str),
	/// fck info
	pub k_info: (&'a str, &'a str),
	/// lint project
	pub k_lint: (&'a str, &'a str),
	/// run some code input
	pub k_raw: (&'a str, &'a str),
	/// build docs
	pub k_doc: (&'a str, &'a str),
	/// translate code
	pub k_translate: (&'a str, &'a str),
}

impl CLICommands<'_> {
	/// number of CLI commands
	pub fn len(&self) -> usize { 10 }
}

impl<'a> Deserialize<'a> for CLICommands<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let cmd_has_spaces = "Command has spaces".to_string();
		let missing_line = "Missing line".to_string();
		macro_rules! field {
			($name:ident) => {
				let $name = {
					let cmd = match s.next() {
						Some(cmd) => if cmd.contains(" ") { return Err(cmd_has_spaces) } else { cmd },
						None => return Err(missing_line)
					};
					let desc = match s.next() {
						Some(desc) => desc,
						None => return Err(missing_line)
					};
					(cmd, desc)
				};
			};
		}
		
		field!(k_new);
		field!(k_shell);
		field!(k_build);
		field!(k_run);
		field!(k_test);
		field!(k_info);
		field!(k_lint);
		field!(k_raw);
		field!(k_doc);
		field!(k_translate);
		Ok(Self { k_new, k_shell, k_build, k_run, k_test, k_info, k_lint, k_raw, k_doc, k_translate })
	}
}

impl<'a> IntoIterator for CLICommands<'a> {
	type Item = (&'a str, &'a str);
	type IntoIter = <Vec<(&'a str, &'a str)> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_new, self.k_shell, self.k_build, self.k_run, self.k_test, self.k_info,
			self.k_lint, self.k_raw, self.k_doc, self.k_translate,
		].into_iter()
	}
}

/// CLI arguments. All values are of the form (long flag, short flag, help)
#[derive(Copy, Clone)]
pub struct CLIArgs<'a> {
	/// help (--help, -h)
	pub k_help: (&'a str, char, &'a str),
	/// path (--path, -p)
	pub k_path: (&'a str, char, &'a str),
	/// git
	pub k_git: (&'a str, char, &'a str),
	/// dump LLVM IR
	pub k_dump_llvm: (&'a str, char, &'a str),
	/// don't build into an object file
	pub k_no_build: (&'a str, char, &'a str),
	/// todo
	pub k_test: (&'a str, char, &'a str),
	/// todo
	pub k_raw: (&'a str, char, &'a str),
	/// target triple
	pub k_target: (&'a str, char, &'a str),
	/// output file
	pub k_output: (&'a str, char, &'a str),
	/// todo
	pub k_comment: (&'a str, char, &'a str),
}

impl CLIArgs<'_> {
	/// number of CLI arguments
	pub fn len(&self) -> usize { 10 }
}

impl<'a> Deserialize<'a> for CLIArgs<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: Deserialize::deserialize(s)?,)* })};
		}
		fields!(k_help, k_path, k_git, k_dump_llvm, k_no_build, k_test, k_raw, k_target, k_output, k_comment)
	}
}

impl<'a> Deserialize<'a> for (&'a str, char, &'a str) {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        if let Some(l) = s.next() {
            let tmp: [&str; 2] = l
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Arg first line pare error: Incorrect number of arguments given");
            Ok((
                tmp[0],
                char::from_str(tmp[1]).expect("Second arg value must be a char"),
                s.next().expect("Expected Arg second line: &str"),
            ))
        } else {
            return Err("Expected Arg first line: &str, char".to_string());
        }
    }
}

impl<'a> IntoIterator for CLIArgs<'a> {
	type Item = (&'a str, char, &'a str);
	type IntoIter = <Vec<(&'a str, char, &'a str)> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_help, self.k_path, self.k_git, self.k_dump_llvm, self.k_no_build,
			self.k_test, self.k_raw, self.k_target, self.k_output, self.k_comment
		].into_iter()
	}
}
