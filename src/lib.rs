pub mod prelude;pub mod languages;pub mod generated;
pub fn get < 'a > (c : & str) -> Result < prelude :: Language < 'a >, () >
{
    match c
    {
        concat! ("\"", stringify! (en), "\"") => Ok(generated :: en :: LANG),
        concat! ("\"", stringify! (de), "\"") => Ok(generated :: de :: LANG),
        _ => Err(())
    }
}