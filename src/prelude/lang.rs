use std::collections::BTreeSet;
use std::convert::TryInto;

use crate::prelude::{TokType, TransitionMap};
use crate::{error, warn};

/// # Generated Language struct
///
/// This defines a language in fck. It includes a name, keywords, errors, warnings, and CLI keys
#[derive(Clone)]
pub struct Language<'a> {
	pub map: &'a [[Option<MapNode<'a>>; 256]],
	pub raw_language: &'a LanguageRaw<'a>,
	pub mapping: TransitionMap<char>,
}

/// # Language mapping node
///
/// For our language maps, each value is a node that can lead to another row and optionally be an
/// accepting state.
pub struct MapNode<'a> {
	pub(in crate::prelude) next: Option<u8>,
	pub(in crate::prelude) accept: Option<TokType<&'a str>>,
}

impl Default for MapNode<'_> {
	fn default() -> Self {
		Self { next: None, accept: None }
	}
}

impl MapNode<'_> {
	pub fn new(next: Option<u8>, accept: Option<TokType<&str>>) -> Self {
		Self { next, accept }
	}
	
	pub fn update<T>(&mut self, other: T) where T: Into<Self> {
		let other = other.into();
		self.next = other.next;
		if let Some(n) = other.accept {
			self.accept = Some(n)
		}
	}
	
	#[inline]
	pub fn index(&self) -> Option<usize> {
		if let Some(v) = self.next { Some(v as usize) } else { None }
	}
}

impl Into<(Option<u8>, Option<TokType<&str>>)> for MapNode<'_> {
	fn into(self) -> (Option<u8>, Option<TokType<&str>>) {
		(self.next, self.accept)
	}
}

/// # Language struct
///
/// This defines a language in fck. It includes a name, keywords, errors, warnings, and CLI keys
#[derive(Clone)]
pub struct LanguageRaw<'a> {
	pub name: (&'a str, &'a str),
	pub keywords: Keywords<'a>,
	pub errors: Errors<'a>,
	pub warnings: Warns<'a>,
	pub cli_keywords: CLIKeywords<'a>,
}

/// # Keywords struct
///
/// Holds all the various keywords and keys for a language
#[derive(Copy, Clone)]
pub struct Keywords<'a> {
	/// General keywords
	pub keywords: [&'a str; 21],
	/// Localised names for built in types and boolean values
	pub builtins: [&'a str; 6],
	/// Boolean constants
	pub bool: [&'a str; 2],
	/// General symbols (string delimiters, comment delimiters, character delimiters,
	/// doc comment symbol)
	pub symbols: [&'a str; 7],
	/// Keys for [`Keywords::symbols`] used when configuring an inherited language
	pub symbol_keys: [&'a str; 7],
	/// Shell keys for use in the config file
	pub shell_keys: [&'a str; 3],
	/// Manifest key values
	pub manifest_keys: [&'a str; 15],
	/// Optional manifest key aliases. For example 'deps' for 'dependencies'
	pub manifest_keys_short: [Option<&'a str>; 15],
	/// Words and phrases used when compiling a project
	pub compile_words: [&'a str; 7],
}

/// # Error message and descriptors
///
/// Holds all the error messages and descriptors. Also includes implementations to make life easy
#[derive(Copy, Clone)]
pub struct Errors<'a> {
	/// Language based errors
	pub e00: [&'a str; 7],
	pub e01: [&'a str; 2],
	pub e02: [&'a str; 9],
	pub e03: [&'a str; 1],
	pub e04: [&'a str; 2],
}

/// # Warning message and descriptors
///
/// Holds all the warning messages and descriptors. Also includes implementations to make life easy
#[derive(Copy, Clone)]
pub struct Warns<'a> {
	pub w00: [&'a str; 0],
	pub w01: [&'a str; 0],
	pub w02: [&'a str; 0],
	pub w03: [&'a str; 0],
	pub w04: [&'a str; 0],
}

/// Holds all the CLI commands, arguments, and help descriptions
///
/// Unfortunately, we have to split up single and double flag arguments. sorry
#[derive(Copy, Clone)]
pub struct CLIKeywords<'a> {
	/// fck CLI description
	pub desc: &'a str,
	/// Commands and help descriptions
	pub commands: [(&'a str, &'a str); 10],
	/// Single flag arguments with help messages
	pub args: [Arg<'a>; 9],
}

/// Argument struct. Used to simplify the argument creation and just make it look nice I guess
#[derive(Copy, Clone)]
pub struct Arg<'a> (pub(crate) &'a str, pub(crate) char, pub(crate) &'a str);

impl LanguageRaw<'_> {
	/// Validates a language to check that the language is correct and usable.
	///
	/// This is where we check for a language violating restrictions we need to place on languages
	/// such as the set of unique characters used being shorter that 219 characters, and not having
	/// identical or conflicting characters
	pub fn validate(&self) -> Result<(), u16> {
		let mut all_check = self.keywords.keywords.to_vec();
		all_check.extend(self.keywords.builtins);
		let l = all_check.len();
		for (p, k_s) in all_check.iter().enumerate() {
			if let Some(f) = all_check[p + 1..l].iter().position(|x| x == k_s) {
				println!("Duplicate found: {}, {}", k_s, all_check[f]);
				return error!(0004, k_s, all_check[f]);
			}
			if k_s.trim() == "" {
				println!("Empty string found: {:?}", k_s);
				return error!(0005);
			}
			for c in "+-%*^/(){}[]!=<>@:?.,;\\\n\t\r ".chars() {
				if k_s.trim().contains(&c.to_string()) {
					println!("Token contains reserved character: {} contains {}", k_s, c);
					return error!(0006);
				}
			}
		}
		let mut char_set = BTreeSet::new();
		for k_s in all_check {
			for c in k_s.chars() { char_set.insert(c); }
		}
		for k_s in self.keywords.symbols {
			for c in k_s.chars() { char_set.insert(c); }
		}
		if char_set.len() > 218 {
			return error!(0007, char_set.len());
		}
		Ok(())
	}
	
	/// Generates a mapping for use in building FSA tables
	pub fn generate_mapping(&self) -> TransitionMap<char> {
		let mut out_iter = vec!['+', '-', '%', '*', '^', '/', '(', ')', '{', '}', '[', ']', '!', '=', '<', '>', '@', ':', '?', '.', ',', ';', '\\', '\n', '\t', '\r', ' '];
		let mut b_set = BTreeSet::new();
		for k in self.keywords.keywords {
			for c in k.chars() {
				b_set.insert(c);
			}
		}
		for k in self.keywords.builtins {
			for c in k.chars() {
				b_set.insert(c);
			}
		}
		for k in self.keywords.bool {
			for c in k.chars() {
				b_set.insert(c);
			}
		}
		for k in self.keywords.symbols {
			for c in k.chars() {
				b_set.insert(c);
			}
		}
		out_iter.extend(b_set.iter());
		TransitionMap::Generate(out_iter)
	}
	
	pub fn kwd_to_maps<'a>(&self) -> Result<(Vec<[MapNode<'a>; 256]>, TransitionMap<char>), u16> {
		macro_rules! single_none {
			($m:ident, $(($i:literal, $n:literal, $t:ident)),+) => {
				$($m[$i][$n] = MapNode::new(None, Some(TokType::$t));)*
			};
		}
		macro_rules! single_update {
			($m:ident, $(($i:literal, $n:literal, $t:ident)),+) => {
				$($m[$i][$n].accept = Some(TokType::$t);)*
			};
		}
		macro_rules! move_none {
			($m:ident, $(($i:literal, $n:literal, $d:literal)),+) => {
				$($m[$i][$n] = MapNode::new(Some($d), None);)*
			};
		}
		macro_rules! move_update {
			($m:ident, $(($i:literal, $n:literal, $d:literal)),+) => {
				$($m[$i][$n].next = Some($d);)*
			};
		}
		let k = self.keywords;
		let char_mapping = self.generate_mapping();
		
		// Check for errors
		if let Err(e) = self.validate() {
			return Err(e);
		}
		
		let key = self.name.1.to_string();
		let mut mapping = vec![[MapNode::default(); 256]; 20];
		
		// Identifier part 1
		mapping[0] = [MapNode::new(Some(15), Some(TokType::Identifier(self.name.1, ""))); 256];
		
		// Single character tokens
		single_none!(mapping,
			(0, 6, LParen), (0, 7, RParen), (0, 8, LParenCurly), (0, 9, RParenCurly),
			(0, 10, LParenSquare), (0, 11, RParenSquare), (0, 17, Colon), (0, 18, QuestionMark),
			(0, 19, Dot), (0, 20, Comma), (0, 23, Newline), (0, 21, Newline)
		);
		
		// Multi characters
		move_none!(mapping,
			(0, 0, 1), // Add
			(0, 1, 2), // Sub
			(0, 2, 3), // Mod
			(0, 3, 4), (4, 3, 5), // Mult
			(0, 5, 6), (6, 5, 7), // Div
			(0, 26, 8), (8, 26, 8), // Space
			(0, 13, 9), // Eq
			(0, 12, 10), // Not
			(0, 14, 11), // LT
			(0, 15, 12) // GT
		);
		single_update!(mapping,
			// Add,Increment,AddSet
			(0, 0, Plus), (1, 0, Increment), (1, 13, SetPlus),
			// Sub,Increment,SubSet
			(0, 1, Minus), (2, 1, Decrement), (2, 13, SetMinus),
			// Mod,SubMod
			(0, 2, Mod), (3, 13, SetMod),
			// Mult,Pow,SetMult,SetPow
			(0, 3, Mult), (4, 3, Pow), (4, 13, SetMult), (5, 13, SetPow),
			// Div,FDiv,SetDiv,SetFDiv
			(0, 5, Div), (6, 5, FDiv), (6, 13, SetDiv), (7, 13, SetFDiv),
			// Space
			(0, 26, Space), (8, 26, Space),
			// Set,Eq
			(0, 13, Set), (9, 13, Eq),
			// Not,NE
			(0, 12, Not), (10, 13, NE),
			// LT,LTE
			(0, 14, LT), (11, 13, LTE),
			// GT,GTE
			(0, 15, GT), (12, 13, GTE)
		);
		
		// Literals
		// Int
		for i in 27..37 {
			mapping[0][i] = MapNode::new(Some(13), Some(TokType::Int(0)));
			mapping[13][i] = MapNode::new(Some(13), Some(TokType::Int(0)));
		}
		// Float
		move_update!(mapping, (0, 19, 14), (13, 19, 14));
		for i in 27..37 {
			mapping[14][i] = MapNode::new(Some(14), Some(TokType::Float(0.)));
		}
		
		// Label
		move_none!(mapping, (0, 16, 17));
		mapping[17] = mapping[0].clone().map(|mut t| if t.next == Some(15) { MapNode::new(Some(17), Some(TokType::Label(""))) } else { t });
		for i in 27..37 {
			move_update!(mapping, (17, i, 17));
			single_update!(mapping, (17, i, Laben("")))
		}
		
		// Identifier part 2
		mapping[15] = mapping[0].map(|t| if t == Some(15) { t } else { None });
		mapping[15][17] = Some(16);
		accepting[15][17] = None;
		mapping[15][26] = None;
		for i in 27..37 {
			mapping[15][i] = Some(15);
		}
		mapping[16] = mapping[15].map(|t| if t == Some(15) { Some(16) } else { None });
		accepting[15] = mapping[15].map(|t| if t == Some(15) {
			Some(TokType::Identifier(key.clone(), String::new()))
		} else { None });
		mapping[16][17] = None;
		accepting[16] = mapping[16].map(|t| if t == Some(16) {
			Some(TokType::Identifier(key.clone(), String::new()))
		} else { None });
		accepting[16][17] = None;
		
		// String
		// Open string
		let mut open_chars = k.symbols[2].chars();
		let mut state = 18;
		let first_byte = char_mapping.run(open_chars.next().unwrap()) as usize;
		mapping[0][first_byte] = Some(state);
		accepting[0][first_byte] = None;
		for b in open_chars {
			mapping.push([None; 256]);
			accepting.push(vec![None; 256].try_into().unwrap());
			mapping[state][char_mapping.run(b) as usize] = Some(state + 1);
			state += 1;
		}
		
		// Inner string
		mapping[state] = [Some(state); 256];
		mapping[state][92] = Some(state + 1);
		mapping[state][10] = None;
		mapping[state + 1] = [Some(state); 256];
		
		// Closing string
		let mut closing_state = state + 2;
		let closing = char_mapping.run(k.symbols[3].chars().last().unwrap()) as usize;
		let closing_chars = k.symbols[3].chars().collect::<Vec<_>>();
		let mut closing_chars = closing_chars[..k.symbols[3].len() - 1].iter();
		if let Some(b) = closing_chars.next() {
			mapping[state][char_mapping.run(*b) as usize] = Some(closing_state);
			closing_state += 1;
			for b in closing_chars {
				mapping.push([None; 256]);
				accepting.push(vec![None; 256].try_into().unwrap());
				mapping[closing_state] = [Some(state); 256];
				mapping[closing_state][char_mapping.run(*b) as usize] = Some(closing_state + 1);
				closing_state += 1;
			}
			accepting[closing_state][closing] = Some(TokType::String(String::new()))
		} else {
			accepting[state][closing] = Some(TokType::String(String::new()))
		}
		
		// Comments
		// Comment start
		let mut open_bytes = k.symbols[0].bytes();
		let first_byte = open_bytes.next().unwrap() as usize;
		mapping[0][first_byte] = Some(state);
		accepting[0][first_byte] = None;
		for b in open_bytes {
			mapping.push([None; 256]);
			accepting.push(vec![None; 256].try_into().unwrap());
			mapping[state][b as usize] = Some(state + 1);
			state += 1;
		}
		
		// Inner inline comment
		mapping[state] = [Some(state); 256];
		mapping[state][10] = None;
		accepting[state][10] = Some(TokType::Comment(key.clone(), String::new()));
		
		let mut return_to_id = [None; 256];
		for i in 27..=255 {
			return_to_id[i] = Some(15);
		}
		for i in 0..4 {
			for c in self.keywords.symbols[i].chars() {
				return_to_id[char_mapping.run(c) as usize] = None
			}
		}
		let accepting_clone: [Option<TokType<String>>; 256] = return_to_id.iter().map(|t|
			if t == &Some(15) { Some(TokType::Identifier(key.clone(), String::new())) } else { None }
		).collect::<Vec<Option<TokType<String>>>>().try_into().unwrap();
		for (k_i, kw) in self.keywords.keywords.iter().enumerate() {
			let last = char_mapping.run(kw.trim().chars().last().unwrap()) as usize;
			let chars = kw.chars().collect::<Vec<char>>();
			let mut chars = chars[..chars.len() - 1].iter();
			if let Some(c) = chars.next() {
				let mut index;
				if mapping[0][char_mapping.run(*c) as usize] == Some(15) || mapping[0][char_mapping.run(*c) as usize].is_none() {
					mapping[0][char_mapping.run(*c) as usize] = Some(mapping.len());
					mapping.push(return_to_id);
					accepting.push(accepting_clone.clone());
					index = mapping.len() - 1
				} else {
					index = mapping[0][char_mapping.run(*c) as usize].unwrap();
				}
				for c_inner in chars {
					if mapping[index][char_mapping.run(*c_inner) as usize] == Some(15) || mapping[index][char_mapping.run(*c_inner) as usize].is_none() {
						mapping[index][char_mapping.run(*c_inner) as usize] = Some(mapping.len());
						mapping.push(return_to_id);
						accepting.push(accepting_clone.clone());
						index = mapping.len() - 1
					} else {
						index = mapping[index][char_mapping.run(*c_inner) as usize].unwrap()
					}
				}
				accepting[index][last] = Some(TokType::Keyword(k_i as u16))
			} else {
				accepting[0][last] = Some(TokType::Keyword(k_i as u16))
			}
		}
		for (k_i, kw) in self.keywords.builtins.iter().enumerate() {
			let last = char_mapping.run(kw.trim().chars().last().unwrap()) as usize;
			let chars = kw.chars().collect::<Vec<char>>();
			let mut chars = chars[..chars.len() - 1].iter();
			if let Some(c) = chars.next() {
				let mut index;
				if mapping[0][char_mapping.run(*c) as usize] == Some(15) || mapping[0][char_mapping.run(*c) as usize].is_none() {
					mapping[0][char_mapping.run(*c) as usize] = Some(mapping.len());
					mapping.push(return_to_id);
					accepting.push(accepting_clone.clone());
					index = mapping.len() - 1
				} else {
					index = mapping[0][char_mapping.run(*c) as usize].unwrap();
				}
				for c_inner in chars {
					if mapping[index][char_mapping.run(*c_inner) as usize] == Some(15) || mapping[index][char_mapping.run(*c_inner) as usize].is_none() {
						mapping[index][char_mapping.run(*c_inner) as usize] = Some(mapping.len());
						mapping.push(return_to_id);
						accepting.push(accepting_clone.clone());
						index = mapping.len() - 1
					} else {
						index = mapping[index][char_mapping.run(*c_inner) as usize].unwrap()
					}
				}
				accepting[index][last] = Some(TokType::Type(k_i as u16))
			} else {
				accepting[0][last] = Some(TokType::Type(k_i as u16))
			}
		}
		for (kw, k_b) in self.keywords.bool.iter().zip([true, false]) {
			let last = char_mapping.run(kw.trim().chars().last().unwrap()) as usize;
			let chars = kw.chars().collect::<Vec<char>>();
			let mut chars = chars[..chars.len() - 1].iter();
			if let Some(c) = chars.next() {
				let mut index;
				if mapping[0][char_mapping.run(*c) as usize] == Some(15) || mapping[0][char_mapping.run(*c) as usize].is_none() {
					mapping[0][char_mapping.run(*c) as usize] = Some(mapping.len());
					mapping.push(return_to_id);
					accepting.push(accepting_clone.clone());
					index = mapping.len() - 1
				} else {
					index = mapping[0][char_mapping.run(*c) as usize].unwrap();
				}
				for c_inner in chars {
					if mapping[index][char_mapping.run(*c_inner) as usize] == Some(15) || mapping[index][char_mapping.run(*c_inner) as usize].is_none() {
						mapping[index][char_mapping.run(*c_inner) as usize] = Some(mapping.len());
						mapping.push(return_to_id);
						accepting.push(accepting_clone.clone());
						index = mapping.len() - 1
					} else {
						index = mapping[index][char_mapping.run(*c_inner) as usize].unwrap()
					}
				}
				accepting[index][last] = Some(TokType::Bool(k_b))
			} else {
				accepting[0][last] = Some(TokType::Bool(k_b))
			}
		}
		Ok((mapping, char_mapping))
	}
}

impl Arg<'static> {
	/// Generate a default standard simple `clap::Arg` struct from this struct
	pub fn clap(self) -> clap::Arg<'static> {
		clap::Arg::new(self.0).long(self.0).short(self.1).help(self.2)
	}
}
