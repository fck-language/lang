use crate::*;
use std::str::FromStr;

macro_rules! fields {
	($s:ident, $($n:ident),*$(,)?) => {$(fields!(@inner, $s, $n);)*};
	(@inner, $s:ident, $n:ident) => {let $n = if let Some(l) = $s.next() {
		if let Ok(res) = l.split_whitespace().collect::<Vec<_>>().try_into() { res } else {
			return Err(concat!("Incorrect number of elements for ", stringify!($n), " line").to_string())
		}
	} else {
		return Err(concat!("Expected ", stringify!($n), " line: [&str: _]").to_string())
	};};
}

pub(crate) trait Deserialize<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String>
    where
        Self: Sized;
}

impl<'a> Deserialize<'a> for LanguageRaw<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        let (left_right, name) = if let Some(l) = s.next() {
            let tmp: [&str; 3] = l
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Unable to parse name: incorrect number of values");
            let lr = match tmp[0] {
                "{" => true,
                "}" => false,
                _ => return Err(format!("{} != '{{' | '}}'", tmp[0]))
            };
            (lr, (tmp[1], tmp[2]))
        } else {
            return Err("Expected name line: (&str, &str)".to_string());
        };
        Ok(Self {
            name, left_right,
            keywords: Keywords::deserialize(s)?,
            messages: Messages::deserialize(s)?,
        })
    }
}

impl<'a> Deserialize<'a> for Messages<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        Ok(Self {
            errors: Errors::deserialize(s)?,
            warnings: Warns::deserialize(s)?,
            cli_keywords: CLIKeywords::deserialize(s)?,
        })
    }
}
