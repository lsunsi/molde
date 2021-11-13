pub fn pascal(s: &str) -> String {
    let mut s = s.to_owned();
    if let Some(c) = s.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
    s
}
