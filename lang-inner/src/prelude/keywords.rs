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
	pub k_set: &'a str,
	pub k_and: &'a str,
	pub k_or: &'a str,
	pub k_not: &'a str,
	pub k_if: &'a str,
	pub k_else: &'a str,
	pub k_match: &'a str,
	pub k_repeat: &'a str,
	pub k_for: &'a str,
	pub k_in: &'a str,
	pub k_to: &'a str,
	pub k_as: &'a str,
	pub k_while: &'a str,
	pub k_fn: &'a str,
	pub k_return: &'a str,
	pub k_continue: &'a str,
	pub k_break: &'a str,
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
	pub k_struct: &'a str,
	pub k_properties: &'a str,
	pub k_enum: &'a str,
	pub k_variants: &'a str,
	pub k_self: &'a str,
	pub k_Self: &'a str,
	pub k_extension: &'a str,
	pub k_extend: &'a str,
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
	pub k_int: &'a str,
	pub k_uint: &'a str,
	pub k_dint: &'a str,
	pub k_udint: &'a str,
	pub k_float: &'a str,
	pub k_bfloat: &'a str,
	pub k_str: &'a str,
	pub k_char: &'a str,
	pub k_list: &'a str,
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
	pub k_true: &'a str,
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
	pub k_package: &'a str,
	pub k_name: &'a str,
	pub k_src: &'a str,
	pub k_tests: &'a str,
	pub k_benches: &'a str,
	pub k_type: &'a str,
	pub k_lib: &'a str,
	pub k_app: &'a str,
	pub k_version: &'a str,
	pub k_authors: &'a str,
	pub k_github: &'a str,
	pub k_gitlab: &'a str,
	pub k_email: &'a str,
	pub k_license: &'a str,
	pub k_description: &'a str,
	pub k_readme: &'a str,
	pub k_homepage: &'a str,
	pub k_repo: &'a str,
	pub k_features: &'a str,
	pub k_dependencies: &'a str,
	pub k_usage: &'a str,
	pub k_git: &'a str,
	pub k_branch: &'a str,
	pub k_path: &'a str,
	pub k_dev: &'a str,
	pub k_build: &'a str,
	pub k_main: &'a str,
}

impl ManifestKwds<'_> {
	pub fn len(&self) -> usize { 27 }
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
			k_dev, k_build, k_main
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
			self.k_branch, self.k_path, self.k_dev, self.k_build, self.k_main,
		].into_iter()
	}
}

/// Compilation (emitted) keywords
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct CompileKwds<'a> {
	pub k_Compiling: &'a str,
	pub k_Building: &'a str,
	pub k_Built: &'a str,
	pub k_Linking: &'a str,
	pub k_Emitted: &'a str,
	pub k_Error: &'a str,
	pub k_errors: &'a str,
	pub k_Warning: &'a str,
	pub k_warning: &'a str,
}

impl<'a> Deserialize<'a> for CompileKwds<'a> {
	fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
		let line = s.next().expect("Expected control keywords line. Found nothing");
		let mut line = line.split_whitespace();
		macro_rules! fields {
		    ($($name:ident),*$(,)?) => {Ok(Self { $($name: line.next().expect(concat!("Expected ", stringify!($name), ", found nothing"))),* })};
		}
		fields!(k_Compiling, k_Building, k_Built, k_Linking, k_Emitted, k_Error, k_errors, k_Warning, k_warning)
	}
}
