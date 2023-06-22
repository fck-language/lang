use crate::{tok::{Position, RunningPosition, Token, PreTokType, PreToken, TokType}, LanguageTuple, LanguageTupleRef};
use lang_inner::{LanguageRaw, Table};
#[cfg(debug_assertions)]
use std::fmt::{Debug, Formatter};

/// # Tokenize an input
///
/// Turn an input into a token stream or return an error in parsing the input
pub fn tokenize<'a, B>(
	mut bytes: B,
	l: &LanguageRaw<'a>,
	buf: &Vec<LanguageTuple<'a>>,
	(transition, tt, td): (&dyn Table<u16>, &dyn Table<u8>, &dyn Table<u8>),
) -> Result<Vec<Token>, u16>
	where
		B: Iterator<Item = u8>,
{
	let mut out = Vec::new();
	let mut bytes = bytes.peekable();
	
	let mut pos = RunningPosition::new();
	let mut tree = NFABranch::new(Vec::new(), pos.clone());
	let mut language_scopes: Vec<LanguageTupleRef> = vec![];
	let mut current_lang = (l, (transition, tt, td));
	
	#[cfg(debug_assertions)]
	macro_rules! debug_dump { () => {
		let w = 200;
		println!("{:-^w$}", format!(" Debug dump {}:{}:{} ", file!(), line!(), column!()));
		for t in out { println!("{:?}", t) }
		println!("{:?}", tree);
		println!("{}", std::backtrace::Backtrace::force_capture());
		println!("{}", "-".repeat(w));
	}; }
	
	#[cfg(not(debug_assertions))]
	macro_rules! debug_dump { () => {}; }
	
	macro_rules! propagate_check {
	    ($t:tt) => {match tree.propagate($t, current_lang.0, current_lang.1.0, current_lang.1.1, current_lang.1.2) {
				NFAPropRes::Continue => {}
				NFAPropRes::End => {
					out.extend(tree.preceding.clone());
					tree.reset(&pos)
				}
				NFAPropRes::Error => {
					if let Some(branch) = &tree.branch {
						tree = *branch.clone()
					} else {
						debug_dump!();
						return Err(0);
					}
				}
			}};
	}
	
	while let Some(b) = bytes.next() {
		#[cfg(debug_assertions)]
		macro_rules! debug_dump { () => {
			let w = 200;
			println!("{:-^w$}", format!(" Debug dump {}:{}:{} ", file!(), line!(), column!()));
			for t in out { println!("{:?}", t) }
			println!("{:?}", tree);
			println!("Current byte: {}", b);
			println!("{}", std::backtrace::Backtrace::force_capture());
			println!("{}", "-".repeat(w));
		}; }
		match b {
			33 => {
				match bytes.next() {
					Some(t) => match t {
						33 => {
							let mut lang_bytes = Vec::new();
							while let Some(b) = bytes.next() {
								if b == 10 || b == 59 { break }
								lang_bytes.push(b)
							}
							pos.advance(10);
							if let Ok(lang_string) = String::from_utf8(lang_bytes) {
								if let Some(ltr) = crate::get(&*lang_string, buf) {
									current_lang = ltr;
								} else {
									debug_dump!();
									return Err(1)
								}
							}
							tree.ps = pos.finish()
						}
						9 | 10 | 32 => {
							out.push(PreToken {
								ps: pos.finish(), pe: pos.finish(),
								tt: PreTokType::Not
							});
							pos.advance(t);
						}
						_ => {
							propagate_check!(33);
							pos.advance(33);
							propagate_check!(t);
							pos.advance(t);
						},
					}
					None => {
						propagate_check!(33);
						pos.advance(33);
						break
					}
				}
			}
			123 => {
				// new scope
				language_scopes.push(current_lang.clone());
				pos.advance(123);
				out.push(PreToken {
					ps: tree.ps, pe: pos.finish(),
					tt: PreTokType::LParenCurly
				});
				tree.ps = pos.finish();
				continue
			}
			125 => {
				// end scope
				if let Some(l) = language_scopes.pop() {
					current_lang = l
				} else {
					debug_dump!();
					return Err(2)
				}
				pos.advance(135);
				out.push(PreToken {
					ps: tree.ps, pe: pos.finish(),
					tt: PreTokType::RParenCurly
				});
				tree.ps = pos.finish();
				continue
			}
			9 | 10 | 32 => {
				pos.advance(b);
				tree.pos.advance(b);
				tree.ps = tree.pos.finish();
				continue
			},
			92 => {
				pos.advance(92);
				match bytes.next() {
					Some(92) => {
						pos.advance(92);
						let mut matched = Vec::new();
						match bytes.next() {
							Some(92) => { /*doc comment */ }
							Some(10) => pos.advance(10),
							Some(t) => {
								pos.advance(t);
								matched.push(t);
								while let Some(n) = bytes.next() {
									pos.advance(n);
									if n == 10 { break }
									matched.push(n);
								}
							}
							None => {}
						}
						out.push(PreToken {
							ps: tree.ps,
							pe: pos.finish(),
							tt: PreTokType::Comment(current_lang.0.name.1.to_string(), matched),
						});
						tree.reset(&pos);
						continue
					}
					Some(42) => {
						pos.advance(42);
						let mut matched = Vec::new();
						while let Some(n) = bytes.next() {
							pos.advance(n);
							matched.push(n);
							if n == 42 {
								match bytes.next() {
									Some(92) => { pos.advance(92); break }
									Some(t) => { pos.advance(t); matched.push(t) }
									None => { return Err(10) }
								}
							}
						}
						matched.pop();
						out.push(PreToken {
							ps: tree.ps,
							pe: pos.finish(),
							tt: PreTokType::Comment(current_lang.0.name.1.to_string(), matched),
						});
						tree.reset(&pos);
						continue
					}
					_ => return Err(9)
				}
			}
			34 => {
				pos.advance(34);
				let mut matched = Vec::new();
				while let Some(t) = bytes.next() {
					pos.advance(t);
					if t == 34 { break }
					if t == 92 {
						let c = parse_char(&mut bytes, &mut pos)?;
						let mut c_u8_4 = [0; 4];
						c.encode_utf8(&mut c_u8_4);
						match c_u8_4 {
							[110, 0, 0, 0] => matched.push(10),
							[116, 0, 0, 0] => matched.push(9),
							[114, 0, 0, 0] => matched.push(13),
							_ => for i in 0..c.len_utf8() { matched.push(c_u8_4[i]) }
						}
					} else { matched.push(t) }
				}
				if pos.previous != 34 {
					debug_dump!();
					return Err(0)
				}
				out.push(PreToken {
					ps: tree.ps,
					pe: pos.finish(),
					tt: PreTokType::String(matched)
				});
				tree.ps = pos.finish();
				continue
			}
			39 => {
				pos.advance(39);
				let c = parse_char(&mut bytes, &mut pos)?;
				
				if bytes.next() != Some(39) {
					debug_dump!();
					return Err(0)
				}
				pos.advance(39);
				out.push(PreToken {
					ps: tree.ps,
					pe: pos.finish(),
					tt: PreTokType::Char(c)
				});
				tree.ps = pos.finish();
				continue
			}
			_ => {
				propagate_check!(b);
				pos.advance(b);
			}
		}
		while let Some(&t) = bytes.peek() {
			#[cfg(debug_assertions)]
			macro_rules! debug_dump { () => {
				let w = 200;
				println!("{:-^w$}", format!(" Debug dump {}:{}:{} ", file!(), line!(), column!()));
				for t in out { println!("{:?}", t) }
				println!("{:?}", tree);
				println!("Current byte: {}", t);
				println!("{}", std::backtrace::Backtrace::force_capture());
				println!("{}", "-".repeat(w));
			}; }
			macro_rules! inside {
			    ($t:ident, $($v:expr),+) => {$($t == $v)||+};
			}
			if inside!(t, 32, 9, 10, 123, 125, 92, 34, 39) {
				match tree.end() {
					Some(rem) => {
						out.extend(rem);
					}
					None => {
						debug_dump!();
						return Err(3);
					},
				}
				pos.advance(t);
				tree.reset(&pos);
				break
			}
			bytes.next();
			match tree.propagate(t, current_lang.0, current_lang.1.0, current_lang.1.1, current_lang.1.2) {
				NFAPropRes::Continue => {}
				NFAPropRes::End => {
					out.extend(tree.preceding.clone());
					tree.preceding.clear();
					tree.branch = None;
					tree.matched.clear();
					pos.advance(t);
					tree.pos = pos.clone();
					tree.ps = pos.finish();
					break
				}
				NFAPropRes::Error => {
					if let Some(branch) = &tree.branch {
						tree = *branch.clone()
					} else {
						debug_dump!();
						return Err(4);
					}
				}
			}
			pos.advance(t);
		}
	}
	
	match tree.end() {
		Some(rem) => {
			out.extend(rem);
			Ok(out.iter().map(|t| Token::from(t)).collect())
		}
		None => {
			debug_dump!();
			return Err(5)
		},
	}
}

/// # Parse character
///
/// Parse a UTF-8 valid character from a byte-stream
fn parse_char<B: Iterator<Item = u8>>(mut bytes: B, pos: &mut RunningPosition) -> Result<char, u16> {
	let mut matched = [0; 4];
	let mut remaining = match bytes.next() {
		Some(t @ 0..=127) => { matched[3] = t; 0 }
		Some(t @ 192..=223) => { matched[3] = t; 1 }
		Some(t @ 224..=239) => { matched[3] = t; 2 }
		Some(t @ 240..=247) => { matched[3] = t; 3 }
		Some(_) => {
			return Err(0)
		}
		None => {
			return Err(0)
		}
	};
	pos.advance(matched[3]);
	
	while remaining > 0 {
		match bytes.next() {
			Some(t @ 128..=191) => {
				matched[3 - remaining] = t;
				pos.advance(t);
			},
			Some(_) => {
				return Err(0)
			}
			None => {
				return Err(0)
			}
		}
		remaining -= 1;
	}
	char::from_u32(u32::from_be_bytes(matched)).ok_or(0)
}

/// # NFA parser
///
/// This struct represents one branch parsing an input through the given transition tables. Each
/// branch parses one token at a time. When a matching token is found, a new child branch is made
/// and assigned to `self.branch`. The current branch continues parsing (until unable to do so)
/// to ensure the matched token is of maximal token length
#[derive(Clone)]
struct NFABranch<'a> {
	/// Current row (state) the branch is on
	row: u16,
	/// Running in-text position
	pos: RunningPosition,
	/// Starting position of the current token being parsed
	ps: Position,
	/// Matched bytes
	matched: Vec<u8>,
	/// Tokens preceding the current position
	preceding: Vec<PreToken<'a>>,
	/// Optional child branch. Only one is needed since if we have a child branch and find a new
	/// token, this is longer than the token matched to start the current child branch and a new
	/// branch can replace it
	branch: Option<Box<Self>>,
}

#[cfg(debug_assertions)]
impl Debug for NFABranch<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut w = f.width().unwrap_or(0);
		write!(f, "{}| row: {}, {:?}->{:?}, preceding: {:?}, matched: {:?}", " ".repeat(w), self.row, self.ps, self.pos, self.preceding, self.matched)?;
		if let Some(child) = &self.branch {
			w += 1;
			write!(f, "\n{:w$?}", child)
		} else { Ok(()) }
	}
}

/// # NFA parsing result
///
/// |  | Reached accepting state | Has not |
/// |---|---|---|
/// | Has a next state | [`Continue`](Self::Continue) | [`Continue`](Self::Continue) |
/// | Does not | [`End`](Self::End) | [`Error`](Self::Error) |
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
enum NFAPropRes {
	/// Node has a next state to go to. Used if the node has branched or not
	Continue,
	/// Node has reached a terminating state and has no next state to go to
	End,
	/// Node has not reached a terminating state and has no next state to go to
	Error,
}

impl<'a> NFABranch<'a> {
	pub fn new(preceding: Vec<PreToken<'a>>, pos: RunningPosition) -> Self {
		let ps = pos.finish();
		Self {
			preceding,
			pos, ps,
			row: 0,
			matched: Vec::new(),
			branch: None,
		}
	}
	
	fn new_branch(&mut self, new: PreToken<'a>) {
		let mut preceding = self.preceding.clone();
		preceding.push(new);
		self.branch = Some(Box::new(NFABranch::new(preceding, self.pos.clone())));
	}
	
	pub fn reset(&mut self, p: &RunningPosition) {
		self.preceding.clear();
		self.branch = None;
		self.row = 0;
		self.matched.clear();
		self.ps = p.finish();
		self.pos = p.clone();
	}
	
	pub fn propagate(&mut self, b: u8, l: &'a LanguageRaw<'a>,
		transition: &dyn Table<u16>, tt: &dyn Table<u8>, td: &dyn Table<u8>) -> NFAPropRes {
		self.pos.advance(b);
		let alternative = if let Some(ref mut branch) = self.branch {
			match branch.propagate(b, l, transition, tt, td) {
				NFAPropRes::Continue => false,
				NFAPropRes::Error => {
					self.branch = branch.branch.clone();
					false
				},
				NFAPropRes::End => true
			}
		} else { false };
		self.matched.push(b);
		let last_row = self.row;
		self.row = transition.element(self.row, b);
		let tt_val = tt.element(last_row, b);
		if self.row == 0 {
			// end of branch i.e. the give byte has no next state in the NFA
			if tt_val == 0 {
				if alternative {
					let branch = self.branch.clone();
					*self = *branch.unwrap();
					return NFAPropRes::End
				}
				// current state is not accepting. branch does not accept input
				NFAPropRes::Error
			} else {
				// current state is accepting. branch accepts input
				self.preceding.push(PreToken::new(
					self.ps.clone(),
					self.pos.finish(),
					tt_val,
					td.element(last_row, b),
					self.matched.clone(),
					l
				));
				NFAPropRes::End
			}
		} else {
			// there is a next state for the given byte
			if tt_val != 0 {
				// this is a branching state i.e. this could be the end of the token/start of a new one
				self.new_branch(
					PreToken::new(
						self.ps.clone(),
						self.pos.finish(),
						tt_val,
						td.element(last_row, b),
						self.matched.clone(),
						l
					)
				)
			}
			NFAPropRes::Continue
		}
	}
	
	pub fn end(&self) -> Option<Vec<PreToken<'a>>> {
		if self.row == 0 {
			Some(self.preceding.clone())
		} else {
			if let Some(branch) = &self.branch {
				branch.end()
			} else {
				None
			}
		}
	}
}

/// QOL function to filter out comments
///
/// This is intended to be used with [`filter`](Iterator::filter) when parsing tokens into ASTs
pub fn comments_filter(tok: &Token) -> bool {
	match tok.tt {
		TokType::Comment(_, _) => false,
		_ => true
	}
}
