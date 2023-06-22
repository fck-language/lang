//! # Number recognition insertion
//!
//! This module provides functions to append rows to the maps to recognise base 10, 2, 16, and 8
//! numbers for [single byte numerals](single_bytes) and a more generic
//! [multi-byte variant](multi_bytes)
//!
//! Matches here will give a `tt=1` and:
//! - `td=0` for base 10
//! - `td=1` for base 2
//! - `td=2` for base 16
//! - `td=3` for base 8

use itertools::Itertools;
use crate::Digits;

pub fn encode(
	digits: &Digits,
	map1: &mut Vec<[u16; 256]>,
	map2: &mut Vec<[u8; 256]>,
	map3: &mut Vec<[u8; 256]>
) {
	match digits {
		Digits::Short{ digits, u8arrays } => if u8arrays.iter().all(|(_, t)| *t == 3) {
			single_bytes(
				digits[3..].iter().map(|t| *t as u8).collect_vec(),
				digits[..3].iter().map(|t| *t as u8).collect(),
				map1, map2, map3
			);
		} else {
			multi_bytes(
				u8arrays[3..].iter().map(|(t, _)| *t).collect(),
				u8arrays[..3].iter().map(|(t, _)| *t).collect(),
				map1, map2, map3
			);
		}
		Digits::Long{ digits, u8arrays } => if u8arrays.iter().all(|(_, t)| *t == 3) {
			single_bytes_long(
				digits[3..].iter().map(|t| *t as u8).collect_vec(),
				digits[..3].iter().map(|t| *t as u8).collect(),
				map1, map2, map3
			);
		} else {
			multi_bytes_long(
				u8arrays[3..].iter().map(|(t, _)| *t).collect(),
				u8arrays[..3].iter().map(|(t, _)| *t).collect(),
				map1, map2, map3
			);
		}
	}
}

/// Single byte numeral table insertion
///
/// This inserts rows to accept all valid digits. Should be used
/// if possible because it's much faster than the [multibyte variant](multi_bytes)
///
/// This returns two `usize`s that are the h0 and h row. This is required to be used by the
/// [long variant](single_bytes_long)
fn single_bytes(
	digits: Vec<u8>, prefixes: Vec<u8>,
	map1: &mut Vec<[u16; 256]>, map2: &mut Vec<[u8; 256]>, map3: &mut Vec<[u8; 256]>
) -> (usize, usize) {
	// d0
	let zero_row = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// q0 --0--> d0
	map1[0][digits[0] as usize] = zero_row as u16;
	map2[0][digits[0] as usize] = 1;
	map3[0][digits[0] as usize] = 2;
	
	// d
	let digit_row = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// d0 --0--> d
	map1[zero_row][digits[0] as usize] = digit_row as u16;
	map2[zero_row][digits[0] as usize] = 1;
	map3[zero_row][digits[0] as usize] = 2;
	for &n in digits[1..10].iter() {
		// q0 --1..9--> d
		map1[0][n as usize] = digit_row as u16;
		map2[0][n as usize] = 1;
		map3[0][n as usize] = 2;
		// d --1..9--> d
		map1[digit_row][n as usize] = digit_row as u16;
		map2[digit_row][n as usize] = 1;
		map3[digit_row][n as usize] = 2;
		// d0 --1..9--> d
		map1[zero_row][n as usize] = digit_row as u16;
		map2[zero_row][n as usize] = 1;
		map3[zero_row][n as usize] = 2;
	}
	// d --0--> d
	map1[digit_row][digits[0] as usize] = digit_row as u16;
	map2[digit_row][digits[0] as usize] = 1;
	map3[digit_row][digits[0] as usize] = 2;
	
	// f
	let float_row = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// {d0,d} --.--> f
	map1[zero_row][46] = float_row as u16;
	map1[digit_row][46] = float_row as u16;
	map2[zero_row][46] = 1;
	map3[zero_row][46] = 6;
	map2[digit_row][46] = 1;
	map3[digit_row][46] = 6;
	// f --0..9--> f
	for &n in digits[0..10].iter() {
		map1[float_row][n as usize] = float_row as u16;
		map2[float_row][n as usize] = 1;
		map3[float_row][n as usize] = 6;
	}
	
	// b0
	let bin_init = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// d0 --b--> b0
	map1[zero_row][prefixes[0] as usize] = bin_init as u16;
	// b
	let bin = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// {b0,b} --0..1--> b
	for &n in digits[0..2].iter() {
		map1[bin_init][n as usize] = bin as u16;
		map1[bin][n as usize] = bin as u16;
		map2[bin_init][n as usize] = 1;
		map3[bin_init][n as usize] = 3;
		map2[bin][n as usize] = 1;
		map3[bin][n as usize] = 3;
	}
	
	// h0
	let hex_init = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// d0 --h--> h0
	map1[zero_row][prefixes[1] as usize] = hex_init as u16;
	// h
	let hex = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// {h0,h} --0..f--> h
	for &n in digits[0..16].iter() {
		map1[hex_init][n as usize] = hex as u16;
		map1[hex][n as usize] = hex as u16;
		map2[hex_init][n as usize] = 1;
		map3[hex_init][n as usize] = 4;
		map2[hex][n as usize] = 1;
		map3[hex][n as usize] = 4;
	}
	
	// o0
	let oct_init = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// d0 --h--> o0
	map1[zero_row][prefixes[2] as usize] = oct_init as u16;
	// o
	let oct = map1.len();
    map1.push([0; 256]);
    map2.push([0; 256]);
    map3.push([0; 256]);
	// {o0,o} --0..7--> o
	for &n in digits[0..8].iter() {
		map1[oct_init][n as usize] = oct as u16;
		map1[oct][n as usize] = oct as u16;
		map2[oct_init][n as usize] = 1;
		map3[oct_init][n as usize] = 5;
		map2[oct][n as usize] = 1;
		map3[oct][n as usize] = 5;
	}
	(hex_init, hex)
}

/// [Long](lang_inner::Digits::Long) digit version of [`single_bytes`]
fn single_bytes_long(
	digits: Vec<u8>, prefixes: Vec<u8>,
	map1: &mut Vec<[u16; 256]>,
	map2: &mut Vec<[u8; 256]>,
	map3: &mut Vec<[u8; 256]>,
) {
	let (hex_init, hex) = single_bytes(digits[..16].to_vec(), prefixes, map1, map2, map3);
	for &n in digits[16..].iter() {
		// {h0,h} --A..F--> h
		map1[hex_init][n as usize] = hex as u16;
		map2[hex_init][n as usize] = 1;
		map3[hex_init][n as usize] = 4;
		map1[hex][n as usize] = hex as u16;
		map2[hex][n as usize] = 1;
		map3[hex][n as usize] = 4;
	}
}

/// todo
pub fn multi_bytes(
	digits: Vec<[u8; 4]>, prefixes: Vec<[u8; 4]>,
	map1: &mut Vec<[u16; 256]>,
	map2: &mut Vec<[u8; 256]>,
	map3: &mut Vec<[u8; 256]>,
) {
	todo!()
}

/// todo
pub fn multi_bytes_long(
	digits: Vec<[u8; 4]>, prefixes: Vec<[u8; 4]>,
	map1: &mut Vec<[u16; 256]>,
	map2: &mut Vec<[u8; 256]>,
	map3: &mut Vec<[u8; 256]>,
) {
	todo!()
}
