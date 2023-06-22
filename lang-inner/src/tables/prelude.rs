use crate::tables::digits;
use crate::LanguageRaw;

pub(crate) const IDENT_ROW: usize = 1;

/// Tabularize a language into three tables
///
/// The tables are used as transition tables for a NFA and correspond to:
/// 1. transition table
/// 2. resultant token type
/// 3. resultant token data
pub fn tabularize(l: &LanguageRaw<'_>) -> (Vec<[u16; 256]>, Vec<[u8; 256]>, Vec<[u8; 256]>) {
    let mut map1: Vec<[u16; 256]> = include!("table_init/map1.in");
    let mut map2: Vec<[u8; 256]> = include!("table_init/map2.in");
    let mut map3: Vec<[u8; 256]> = include!("table_init/map3.in");
	if !l.left_right {
		// swap brackets
		let temp = map3[0][40];
		map3[0][40] = map3[0][41];
		map3[0][41] = temp;
		let temp = map3[0][91];
		map3[0][91] = map3[0][93];
		map3[0][93] = temp;
	}

	// clone map1[0] to be map1[IDENT_ROW]
	let mut repeat_ident_row = [0; 256];
	let mut repeat_ident_row_m2 = [0; 256];
	for i in 0..256 {
		if map2[0][i] == 0 {
			if map1[0][i] == 0 { repeat_ident_row[i] = IDENT_ROW as u16; }
			repeat_ident_row_m2[i] = 7;
		}
	}
	for i in [9, 10, 32, 123, 125] {
		repeat_ident_row[i] = 0;
		repeat_ident_row_m2[i] = 0;
	}

	digits::encode(&l.keywords.digits, &mut map1, &mut map2, &mut map3);

    // for each keyword, follow it as far as the table currently has
    // if we need to add additional rows, we add them
    // when we finish, set the corresponding map2 and map3 values
    // these are guaranteed to be both zero since the language will have been checked for
    // collisions earlier on

    let mut ident_rows: Vec<usize> = Vec::new();
    
    macro_rules! simple_map {
		($f:expr, $($t: expr),*$(,)?) => {let mut counter = 0;simple_map!(@inner, $f, counter);$(counter += 1; simple_map!(@inner, $t, counter);)*};
	    (@inner, $t: expr, $e: ident) => {
			for (td, kwd) in $t.iter().enumerate() {
				let mut row_index = 0;
				let (last, split) = kwd.as_bytes().split_last().unwrap();
				for b in split {
					if map1[row_index][*b as usize] == 0 {
						// insert a new row
						map1[row_index][*b as usize] = map1.len() as u16;
						row_index = map1.len() as usize;
						ident_rows.push(row_index);
						map1.push([0; 256]);
						map2.push(repeat_ident_row_m2);
						map3.push([0; 256]);
					} else {
						// otherwise move through the table
						row_index = map1[row_index][*b as usize] as usize;
					}
				}
				map2[row_index][*last as usize] = 6;
				map3[row_index][*last as usize] = ($e << 6) + td as u8;
			}
		};
	}
	
    simple_map!(
        l.keywords.keywords,
        l.keywords.type_kwds,
        l.keywords.builtins,
        l.keywords.bool
    );
	
	for i in ident_rows {
		// merge current row with ident_row with the current row taking priority
		for n in 0..256 {
			if map1[i][n] == 0 { map1[i][n] = repeat_ident_row[n] }
		}
	}
	
	map1[IDENT_ROW] = repeat_ident_row;
	map2[IDENT_ROW] = repeat_ident_row_m2;
	
	for i in 0..256 {
		if map2[0][i] == 0 {
			if map1[0][i] == 0 { map1[0][i] = IDENT_ROW as u16; }
			map2[0][i] = 7;
		}
	}
	
	for i in [9, 10, 32, 123, 125] {
		map1[0][i] = 0;
		map2[0][i] = 0;
	}

    (map1, map2, map3)
}
