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
	pub mapping: TransitionMap<char>
}

/// # Language mapping node
///
/// For our language maps, each value is a node that can lead to another row and optionally be an
/// accepting state.
pub struct MapNode<'a> {
	next: Option<u8>,
	accept: Option<TokType<&'a str>>
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
	
	pub fn update(&mut self, other: Self) {
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
			if let Some(f) = all_check[p+1..l].iter().position(|x| x == k_s) {
				println!("Duplicate found: {}, {}", k_s, all_check[f]);
				return error!(0004, k_s, all_check[f])
			}
			if k_s.trim() == "" {
				println!("Empty string found: {:?}", k_s);
				return error!(0005)
			}
			for c in "+-%*^/(){}[]!=<>@:?.,;\\\n\t\r ".chars() {
				if k_s.trim().contains(&c.to_string()) {
					println!("Token contains reserved character: {} contains {}", k_s, c);
					return error!(0006)
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
			return error!(0007, char_set.len())
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
		let k = self.keywords;
		let char_mapping = self.generate_mapping();
		
		// Check for errors
		if let Err(e) = self.validate() {
			return Err(e)
		}
		
		let key = self.name.1.to_string();
		let mut mapping = vec![[MapNode::default(); 256]; 20];
		
		// Identifier part 1
		mapping[0] = [MapNode::new(Some(15), Some(TokType::Identifier(&*key.clone(), ""))); 256];
		accepting[0] = vec![Some(TokType::Identifier(key.clone(), String::new())); 256].try_into().unwrap();
		
		// Single character tokens
		// LParen
		mapping[0][6] = MapNode::new(None, Some(TokType::LParen));
		// RParen
		accepting[0][7] = Some(TokType::RParen);
		mapping[0][7] = MapNode::new(None, Some(TokType::RParen));
		// LParenCurly
		accepting[0][8] = Some(TokType::LParenCurly);
		mapping[0][8] = MapNode::new(None, Some(TokType::LParenCurly));
		// RParenCurly
		accepting[0][9] = Some(TokType::RParenCurly);
		mapping[0][9] = MapNode::new(None, Some(TokType::RParenCurly));
		// LParenSquare
		accepting[0][10] = Some(TokType::LParenSquare);
		mapping[0][10] = MapNode::new(None, Some(TokType::LParenSquare));
		// RParenSquare
		accepting[0][11] = Some(TokType::RParenSquare);
		mapping[0][11] = MapNode::new(None, Some(TokType::RParenSquare));
		// Colon
		accepting[0][17] = Some(TokType::Colon);
		mapping[0][17] = MapNode::new(None, Some(TokType::Colon));
		// QuestionMark
		accepting[0][18] = Some(TokType::QuestionMark);
		mapping[0][18] = None;
		// Dot
		accepting[0][19] = Some(TokType::Dot);
		mapping[0][19] = None;
		// Comma
		accepting[0][20] = Some(TokType::Comma);
		mapping[0][20] = None;
		// Newline
		accepting[0][23] = Some(TokType::Newline);
		mapping[0][23] = None;
		accepting[0][21] = Some(TokType::Newline);
		mapping[0][21] = None;
		
		// Multi characters
		// Add,Increment,AddSet
		mapping[0][0] = Some(1);
		accepting[0][0] = Some(TokType::Plus);
		accepting[1][0] = Some(TokType::Increment);
		accepting[1][13] = Some(TokType::SetPlus);
		// Sub,Increment,SubSet
		mapping[0][1] = Some(2);
		accepting[0][1] = Some(TokType::Minus);
		accepting[2][1] = Some(TokType::Decrement);
		accepting[2][13] = Some(TokType::SetMinus);
		// Mod,SubMod
		mapping[0][2] = Some(3);
		accepting[0][2] = Some(TokType::Mod);
		accepting[3][13] = Some(TokType::SetMod);
		// Mult,Pow,SetMult,SetPow
		mapping[0][3] = Some(4);
		accepting[0][3] = Some(TokType::Mult);
		accepting[4][3] = Some(TokType::Pow);
		mapping[4][3] = Some(5);
		accepting[4][13] = Some(TokType::SetMult);
		accepting[5][13] = Some(TokType::SetPow);
		// Div,FDiv,SetDiv,SetFDiv
		mapping[0][5] = Some(6);
		accepting[0][5] = Some(TokType::Div);
		accepting[6][5] = Some(TokType::FDiv);
		mapping[6][5] = Some(7);
		accepting[6][13] = Some(TokType::SetDiv);
		accepting[7][13] = Some(TokType::SetFDiv);
		// Space
		mapping[0][26] = Some(8);
		accepting[0][26] = Some(TokType::Space);
		mapping[8][26] = Some(8);
		accepting[8][26] = Some(TokType::Space);
		// Set,Eq
		mapping[0][13] = Some(9);
		accepting[0][13] = Some(TokType::Set);
		accepting[9][13] = Some(TokType::Eq);
		// Not,NE
		mapping[0][12] = Some(10);
		accepting[0][12] = Some(TokType::Not);
		accepting[10][13] = Some(TokType::NE);
		// LT,LTE
		mapping[0][14] = Some(11);
		accepting[0][14] = Some(TokType::LT);
		accepting[11][13] = Some(TokType::LTE);
		// GT,GTE
		mapping[0][15] = Some(12);
		accepting[0][15] = Some(TokType::GT);
		accepting[12][13] = Some(TokType::GTE);
		
		// Literals
		// Int
		for i in 27..37 {
			mapping[0][i] = Some(13);
			accepting[0][i] = Some(TokType::Int(0));
			mapping[13][i] = Some(13);
			accepting[13][i] = Some(TokType::Int(0));
		}
		// Float
		mapping[0][19] = Some(14);
		mapping[13][19] = Some(14);
		accepting[13][19] = Some(TokType::Float(0.));
		for i in 27..37 {
			mapping[14][i] = Some(14);
			accepting[14][i] = Some(TokType::Float(0.));
		}
		
		// Label
		mapping[0][16] = Some(17);
		accepting[0][16] = None;
		mapping[17] = mapping[0].map(|t| if t == Some(15) { Some(17) } else { None });
		for i in 27..37 {
			mapping[17][i] = Some(17);
		}
		accepting[17] = mapping[17].map(|t| if t == Some(17) {
			Some(TokType::Label(String::new()))
		} else { None });
		
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
						index  = mapping.len() - 1
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
						index  = mapping.len() - 1
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
						index  = mapping.len() - 1
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
