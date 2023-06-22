use lang;

#[test]
fn correct_formatted() {
    let buf = Vec::new();
    for i in ["eN", "En", "EN"] {
        assert!(lang::get(i, &buf).is_some());
    }
}

#[cfg(test)]
mod incorrect_codes {
    #[test]
    fn too_long() {
        let buf = Vec::new();
        assert!(lang::get("___", &buf).is_none());
    }
    #[test]
    fn too_short() {
        let buf = Vec::new();
        assert!(lang::get("_", &buf).is_none());
    }
}
