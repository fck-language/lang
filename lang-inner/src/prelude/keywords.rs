//! Keywords struct and all it's constituent structs

use std::str::FromStr;
use crate::de::Deserialize;

/// # Keywords struct
///
/// Holds all the various keywords and keys for a language
#[derive(Clone)]
pub struct Keywords<'a> {
    /// Digits and number prefixes
    pub digits: Digits,
    /// General keywords
    pub keywords: ControlKwds<'a>,
    /// General keywords
    pub type_kwds: TypeKwds<'a>,
    /// Localised names for built in types and boolean values
    pub builtins: PrimitiveKwds<'a>,
    /// Boolean constants
    pub bool: BoolKwds<'a>,
    /// Manifest key values
    pub manifest_keys: ManifestKwds<'a>,
    /// Words and phrases used when compiling a project
    pub compile_words: CompileKwds<'a>,
}

impl<'a> Deserialize<'a> for Keywords<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => { Ok(Self { $($name: Deserialize::deserialize(s)?),* }) };
		}
		fields!(digits, keywords, type_kwds, builtins, bool, manifest_keys, compile_words)
	}
}

/// # Digits type
///
/// See [DigitsRaw]
#[derive(Clone)]
pub enum Digits {
    /// Short version of digits with no uppercase variants for ten to fifteen characters
    Short(DigitsRaw<16>),
    /// Long version of digits with uppercase variants for ten to fifteen characters
	Long(DigitsRaw<22>)
}

impl<'a> Deserialize<'a> for Digits {
    fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
        if let Some(ln) = s.next() {
            let mut digits = Vec::with_capacity(26);
            for d in ln.split_whitespace() {
                if let Ok(d) = char::from_str(d) {
                    digits.push(d)
                } else {
                    return Err(format!("{:?} is not a character", d))
                }
            }
            let digits_u8array = digits.iter().map(|t| {
                let temp = (*t as u32).to_be_bytes();
                let f = temp.iter().skip_while(|&t| *t == 0).count();
                (temp, 4 - f as u8)
            }).collect::<Vec<_>>();
            match digits.len() {
                19 => Ok(Digits::Short(DigitsRaw {
					bin_pre: digits[0],
					bin_pre_u8: digits_u8array[0],
					hex_pre: digits[1],
					hex_pre_u8: digits_u8array[1],
					oct_pre: digits[2],
					oct_pre_u8: digits_u8array[2],
					digits: digits[3..].to_vec().try_into().unwrap(),
					u8arrays: digits_u8array[3..].to_vec().try_into().unwrap(),
				})),
                25 => Ok(Digits::Long(DigitsRaw {
					bin_pre: digits[0],
					bin_pre_u8: digits_u8array[0],
					hex_pre: digits[1],
					hex_pre_u8: digits_u8array[1],
					oct_pre: digits[2],
					oct_pre_u8: digits_u8array[2],
					digits: digits[3..].to_vec().try_into().unwrap(),
					u8arrays: digits_u8array[3..].to_vec().try_into().unwrap(),
				})),
                t => Err(format!("Digits must be either 19 or 25 long. Found {}", t))
            }
        } else {
            Err(format!("Expected digits line"))
        }
    }
}

impl Digits {
    /// Separate a matched digit (`Vec<u8>`) into it's component digit units.
    ///
    /// For example, if we had `vec![50, 48, 51]` for 203, this would separate that into `vec![2, 0, 3]`
    pub fn separate(&self, base: usize, matcher: Vec<u8>) -> Vec<u8> {
        let pos = match self {
            Digits::Short(t) => t.u8arrays[..base].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>(),
            Digits::Long(t) => if base == 16 {
                let mut out = t.u8arrays[..16].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>();
                out.extend((10usize..16).zip(t.u8arrays[16..].iter().map(|(t, s)| (*t, *s as usize))));
                out
            } else {
                t.u8arrays[..base].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>()
            }
        };
        let mut pos = pos.iter().cycle();
        let mut n = 0;
        let mut out = Vec::new();
        while n < matcher.len() {
            if let Some((v, (_, s))) = pos.find(|(_, (t, s))| &t[*s..] == &matcher[n..n + 4 - *s]) {
                out.push(*v as u8);
                n += 4 - s;
            } else { unreachable!("Unable to match digit bytes with digits:\n{:?}\n{:?}", matcher, pos.collect::<Vec<_>>()) }
        }
        out
    }
}

/// Raw digits
///
/// Contains the binary, hex, and octal prefixes; as well as digits.
/// u8 array versions (chars cast to `u32` with their length in bytes) are also given
#[derive(Clone)]
pub struct DigitsRaw<const N: usize> {
	/// Binary prefix
	pub bin_pre: char,
	/// u8 array binary prefix
	pub bin_pre_u8: ([u8; 4], u8),
	/// Hexidecimal prefix
	pub hex_pre: char,
	/// u8 array hexidecimal prefix
	pub hex_pre_u8: ([u8; 4], u8),
	/// Octal prefix
	pub oct_pre: char,
	/// u8 array octal prefix
	pub oct_pre_u8: ([u8; 4], u8),
	/// Digit literals
	pub digits: [char; N],
	/// u8 arrays of digit literals
	pub u8arrays: [([u8; 4], u8); N],
}

/// All the control keywords
#[derive(Copy, Clone)]
pub struct ControlKwds<'a> {
	/// `set` keyword\
	/// variable definitions
	pub k_set: &'a str,
	/// `and` keyword\
	/// boolean operator. equivalent to `&&`
	pub k_and: &'a str,
	/// `or` keyword\
	/// boolean operator. equivalent to `|`
	pub k_or: &'a str,
	/// `not` keyword\
	/// boolean operator. equivalent to `!`
	pub k_not: &'a str,
	/// `if` keyword
	pub k_if: &'a str,
	/// `else` keyword
	pub k_else: &'a str,
	/// `match` keyword
	pub k_match: &'a str,
	/// `repeat` keyword\
	/// conditional operator. repeats a block some number of times
	pub k_repeat: &'a str,
	/// `for` keyword
	pub k_for: &'a str,
	/// `in` keyword\
	/// used as part of `for` statement
	pub k_in: &'a str,
	/// `to` keyword\
	/// range operator
	pub k_to: &'a str,
	/// `as` keyword\
	/// import name modifier (`import ?? as ??`) or extension specifier (`<Ty as Ext>::fn`)
	pub k_as: &'a str,
	/// `while` keyword\
	/// conditional operator
	pub k_while: &'a str,
	/// `fn` keyword\
	/// function definition operator
	pub k_fn: &'a str,
	/// `return` keyword
	pub k_return: &'a str,
	/// `continue` keyword
	pub k_continue: &'a str,
	/// `break` keyword
	pub k_break: &'a str,
	/// `where` keyword\
	/// extension constrain keyword
	pub k_where: &'a str,
}

impl<'a> Deserialize<'a> for ControlKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(
			k_set, k_and, k_or, k_not, k_if, k_else, k_match, k_repeat, k_for, k_in,
			k_to, k_as, k_while, k_fn, k_return, k_continue, k_break, k_where
		)
	}
}

impl<'a> IntoIterator for ControlKwds<'a> {
	type Item = &'a str;
	type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_set, self.k_and, self.k_or, self.k_not, self.k_if, self.k_else, self.k_match,
			self.k_repeat, self.k_for, self.k_in, self.k_to, self.k_as, self.k_while, self.k_fn,
			self.k_return, self.k_continue, self.k_break, self.k_where
		].into_iter()
	}
}

/// All the type keywords
#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct TypeKwds<'a> {
	/// `struct` type keyword
	pub k_struct: &'a str,
	/// `properties` of a `struct`
	pub k_properties: &'a str,
	/// `enum` type keyword
	pub k_enum: &'a str,
	/// `variants` of an `enum`
	pub k_variants: &'a str,
	/// `self` keyword\
	/// Specific instance being acted upon
	pub k_self: &'a str,
	/// `Self` keyword\
	/// Alias for the type of `self`
	pub k_Self: &'a str,
	/// `extension` keyword
	pub k_extension: &'a str,
	/// `extend` keywors
	pub k_extend: &'a str,
	/// `const` type keyword
	pub k_const: &'a str,
}

impl<'a> Deserialize<'a> for TypeKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(k_struct, k_properties, k_enum, k_variants, k_self, k_Self, k_extension, k_extend, k_const)
	}
}

impl<'a> IntoIterator for TypeKwds<'a> {
	type Item = &'a str;
	type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_struct, self.k_properties, self.k_enum, self.k_variants, self.k_self, self.k_Self, self.k_extension, self.k_extend, self.k_const
		].into_iter()
	}
}

/// All the primitive names
#[derive(Copy, Clone)]
pub struct PrimitiveKwds<'a> {
	/// integer type (`isize` in Rust)
	pub k_int: &'a str,
	/// unsigned integer type (`usize` in Rust)
	pub k_uint: &'a str,
	/// dynamic integer type
	pub k_dint: &'a str,
	/// unsigned dynamic integer type
	pub k_udint: &'a str,
	/// float type (`f32` or `f64` depending on platform)
	pub k_float: &'a str,
	/// float stored in base 10 (two `int` types for mantissa and exponent)
	pub k_bfloat: &'a str,
	/// string type
	pub k_str: &'a str,
	/// character type (Unicode Scalar Value)
	pub k_char: &'a str,
	/// List type
	pub k_list: &'a str,
	/// Boolean type
	pub k_bool: &'a str,
}

impl<'a> Deserialize<'a> for PrimitiveKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(k_int, k_uint, k_dint, k_udint, k_float, k_bfloat, k_str, k_char, k_list, k_bool)
	}
}

impl<'a> IntoIterator for PrimitiveKwds<'a> {
	type Item = &'a str;
	type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_int, self.k_uint, self.k_dint, self.k_udint, self.k_float, self.k_bfloat,
			self.k_str, self.k_char, self.k_list, self.k_bool
		].into_iter()
	}
}

/// Boolean true and false values
#[derive(Copy, Clone)]
pub struct BoolKwds<'a> {
	/// `true` keyword
	pub k_true: &'a str,
	/// `false` keyword
	pub k_false: &'a str,
}

impl<'a> Deserialize<'a> for BoolKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(k_true, k_false)
	}
}

impl<'a> IntoIterator for BoolKwds<'a> {
	type Item = &'a str;
	type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_true, self.k_false
		].into_iter()
	}
}

/// Manifest file keys
#[derive(Copy, Clone)]
pub struct ManifestKwds<'a> {
	/// `package` keyword
	pub k_package: &'a str,
	/// `name` keyword\
	/// name of the package
	pub k_name: &'a str,
	/// `src` keyword\
	/// path to folder with source code in
	pub k_src: &'a str,
	/// `tests` keyword\
	/// path to folder with tests in
	pub k_tests: &'a str,
	/// `benches` keyword\
	/// path to folder with benchmarks in
	pub k_benches: &'a str,
	/// `type` keyword\
	/// parent node of specifying package type(s)
	pub k_type: &'a str,
	/// `lib` keyword\
	/// specified path (relative to `src`) of library file
	pub k_lib: &'a str,
	/// `lib` keyword\
	/// specified path (relative to `src`) of application file
	pub k_app: &'a str,
	/// `version` keyword\
	/// sem-ver package version
	pub k_version: &'a str,
	/// `authors` keyword\
	/// parent node for authors of package
	pub k_authors: &'a str,
	/// `github` keyword\
	/// github account for author
	pub k_github: &'a str,
	/// `gitlab` keyword\
	/// gitlab account for author
	pub k_gitlab: &'a str,
	/// `email` keyword\
	/// github address for author
	pub k_email: &'a str,
	/// `license` keyword\
	/// license for package
	pub k_license: &'a str,
	/// `description` keyword\
	/// description of the package
	pub k_description: &'a str,
	/// `readme` keyword\
	/// path to README file
	pub k_readme: &'a str,
	/// `homepage` keyword\
	/// package homepage
	pub k_homepage: &'a str,
	/// `repo` keyword\
	/// package repo
	pub k_repo: &'a str,
	/// `features` keyword\
	/// parent node for features for package
	pub k_features: &'a str,
	/// `dependencies` keyword\
	///	dependencies node for, dependencies
	pub k_dependencies: &'a str,
	/// `usage` keyword\
	/// usage section for usage of dependency
	pub k_usage: &'a str,
	/// `git` keyword\
	///	link to git repo for dependency
	pub k_git: &'a str,
	/// `branch` keyword\
	///	branch for a git dependency
	pub k_branch: &'a str,
	/// `path` keyword\
	///	path for dependency
	pub k_path: &'a str,
	/// `dev` keyword\
	///	dev dependencies node
	pub k_dev: &'a str,
	/// `build` keyword\
	/// build dependencies node
	pub k_build: &'a str,
}

impl ManifestKwds<'_> {
	/// Number of manifest keywords
	pub fn len(&self) -> usize { 26 }
}

impl<'a> Deserialize<'a> for ManifestKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(
			k_package, k_name, k_src, k_tests, k_benches, k_type, k_lib, k_app, k_version,
			k_authors, k_github, k_gitlab, k_email, k_license, k_description, k_readme,
			k_homepage,k_repo, k_features, k_dependencies, k_usage, k_git, k_branch, k_path,
			k_dev, k_build
		)
	}
}

impl<'a> IntoIterator for ManifestKwds<'a> {
	type Item = &'a str;
	type IntoIter = <Vec<&'a str> as IntoIterator>::IntoIter;
	
	fn into_iter(self) -> Self::IntoIter {
		vec![
			self.k_package, self.k_name, self.k_src, self.k_tests, self.k_benches, self.k_type,
			self.k_lib, self.k_app, self.k_version, self.k_authors, self.k_github, self.k_gitlab,
			self.k_email, self.k_license, self.k_description, self.k_readme, self.k_homepage,
			self.k_repo, self.k_features, self.k_dependencies, self.k_usage, self.k_git,
			self.k_branch, self.k_path, self.k_dev, self.k_build,
		].into_iter()
	}
}

/// Compilation (emitted) keywords
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct CompileKwds<'a> {
	/// 'compiling'
	pub k_Compiling: &'a str,
	/// 'building'
	pub k_Building: &'a str,
	/// 'built'
	pub k_Built: &'a str,
	/// 'linking'
	pub k_Linking: &'a str,
	/// 'emitted'\
	/// for number of errors and warnings
	pub k_Emitted: &'a str,
	/// 'error'
	pub k_Error: &'a str,
	/// 'errors'
	pub k_errors: &'a str,
	/// 'warning'
	pub k_Warning: &'a str,
	/// 'warnings'
	pub k_warnings: &'a str,
}

impl<'a> Deserialize<'a> for CompileKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(k_Compiling, k_Building, k_Built, k_Linking, k_Emitted, k_Error, k_errors, k_Warning, k_warnings)
	}
}
