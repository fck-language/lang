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

impl<'a> Deserialize<'a> for Keywords<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        let digits = Digits::deserialize(s)?;
        fields!(
            s,
            keywords,
            type_kwds,
            builtins,
            bool,
            symbol_keys,
            shell_keys,
            manifest_keys
        );
        let manifest_keys_short = if let Some(l) = s.next() {
            let temp: [&str; 15] = l
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Unable to parse manifest_keys_short: Incorrect number of arguments");
            temp.map(|t| if t == "_" { None } else { Some(t) })
        } else {
            return Err("Expected manifest_keys_short line: [Option<&str>; 15]".to_string());
        };
        fields!(s, compile_words);
        Ok(Self {
            digits,
            keywords,
            type_kwds,
            builtins,
            bool,
            symbol_keys,
            shell_keys,
            manifest_keys,
            manifest_keys_short,
            compile_words,
        })
    }
}

impl<'a> Deserialize<'a> for Digits {
    fn deserialize<T: Iterator<Item=&'a str>>(s: &mut T) -> Result<Self, String> where Self: Sized {
        if let Some(ln) = s.next() {
            let mut digits = Vec::with_capacity(25);
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
                19 => Ok(Digits::Short {
                    digits: digits.try_into().unwrap(),
                    u8arrays: digits_u8array.try_into().unwrap()
                }),
                25 => Ok(Digits::Long {
                    digits: digits.try_into().unwrap(),
                    u8arrays: digits_u8array.try_into().unwrap()
                }),
                t => Err(format!("Digits must be either 19 or 25 long. Found {}", t))
            }
        } else {
            Err(format!("Expected digits line"))
        }
    }
}

impl<'a> Deserialize<'a> for Errors<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        let e00 = (0..7)
            .map(|i| s.next().expect(&*format!("Expected E00{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let e01 = (0..2)
            .map(|i| s.next().expect(&*format!("Expected E01{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let e02 = (0..9)
            .map(|i| s.next().expect(&*format!("Expected E02{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let e03 = (0..1)
            .map(|i| s.next().expect(&*format!("Expected E03{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let e04 = (0..2)
            .map(|i| s.next().expect(&*format!("Expected E04{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Self {
            e00,
            e01,
            e02,
            e03,
            e04,
        })
    }
}

impl<'a> Deserialize<'a> for Warns<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        let w00 = (0..0)
            .map(|i| s.next().expect(&*format!("Expected W00{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let w01 = (0..0)
            .map(|i| s.next().expect(&*format!("Expected W01{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let w02 = (0..0)
            .map(|i| s.next().expect(&*format!("Expected W02{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let w03 = (0..0)
            .map(|i| s.next().expect(&*format!("Expected W03{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let w04 = (0..0)
            .map(|i| s.next().expect(&*format!("Expected W04{:>20}", i)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Self {
            w00,
            w01,
            w02,
            w03,
            w04,
        })
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

impl<'a> Deserialize<'a> for CLIKeywords<'a> {
    fn deserialize<T: Iterator<Item = &'a str>>(s: &mut T) -> Result<Self, String> {
        let desc = if let Some(l) = s.next() {
            l
        } else {
            return Err("Expected CLI desc: &str".to_string());
        };
        let commands = [("", ""); 10].map(|_| {
            (
                s.next().expect("Expected CLI command: &str"),
                s.next().expect("Expected CLI desc: &str"),
            )
        });
        let mut args = [("", char::MAX, ""); 10];
        for i in 0..10 {
            args[i] = Deserialize::deserialize(s)?
        }
        Ok(Self {
            desc,
            commands,
            args,
        })
    }
}
