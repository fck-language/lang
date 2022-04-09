use lang;

#[test]
fn correct_code() {
    for i in ["en", "de", "fr", "ko"] {
        assert!(lang::get_associated_keywords(i).is_some());
        assert!(lang::get_associated_messages(i).is_some())
    }
}

#[cfg(test)]
mod incorrect_codes {
    #[test]
    fn too_long() {
        assert!(lang::get_associated_keywords("___").is_none());
        assert!(lang::get_associated_messages("___").is_none())
    }
    #[test]
    fn too_short() {
        assert!(lang::get_associated_keywords("_").is_none());
        assert!(lang::get_associated_messages("_").is_none())
    }
}
