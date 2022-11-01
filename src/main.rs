use std::io::Write;mod prelude;mod languages;
fn main() -> Result < (), std :: io :: Error >
{
    std :: fs :: write("src/generated/mod.rs", "") ? ; let mut a = std :: fs
    :: OpenOptions ::
    new().write(true).append(true).open("src/generated/mod.rs") ? ;
    let(map, acc, c_map) = languages :: en :: LANG.kwd_to_maps().unwrap() ;
    std :: fs ::
    write(concat! ("src/generated/", stringify! (en), ".rs"), format!
    ("use crate::prelude::{{TokType::*, Language, TransitionMap}};\npub const LANG: Language = Language {{ transition_map: &{:?}, accepting_map: &{:?}, raw_language: &crate::languages::{}::LANG, mapping: TransitionMap::Const({}) }};",
    map, acc, stringify! (en), c_map.to_string())) ? ; write!
    (a, concat! ("pub mod ", stringify! (en), ";")) ? ; let(map, acc, c_map) =
    languages :: de :: LANG.kwd_to_maps().unwrap() ; std :: fs ::
    write(concat! ("src/generated/", stringify! (de), ".rs"), format!
    ("use crate::prelude::{{TokType::*, Language, TransitionMap}};\npub const LANG: Language = Language {{ transition_map: &{:?}, accepting_map: &{:?}, raw_language: &crate::languages::{}::LANG, mapping: TransitionMap::Const({}) }};",
    map, acc, stringify! (de), c_map.to_string())) ? ; write!
    (a, concat! ("pub mod ", stringify! (de), ";")) ? ; std :: fs ::
    write("src/lib.rs", concat!
    ("pub mod prelude;pub mod languages;pub mod generated;\n", stringify!
    (pub fn get < 'a > (c : & str) -> Result < prelude :: Language < 'a >, ()
    >
    {
        match c
        {
            concat! ("\"", stringify! (en), "\"") =>
            Ok(generated :: en :: LANG), concat! ("\"", stringify! (de), "\"")
            => Ok(generated :: de :: LANG), _ => Err(())
        }
    }))) ? ; Ok(())
}