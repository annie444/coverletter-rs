pub fn capitalize(s: &str) -> String {
    let mut f = String::new();
    for c in s.chars() {
        f.push(c.to_ascii_uppercase());
    }
    f
}
