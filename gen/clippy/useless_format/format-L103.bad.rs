fn main() {
    let _: &str = &format!("hello"); //~ useless_format
    let _: &str = &format!("{{}}"); //~ useless_format
    let _: &str = &format!(r"hello"); //~ useless_format
    let _: &str = &format!(r#"world"#); //~ useless_format
    let _: &str = &format!(r##"foo"##); //~ useless_format
    let _: &str = &format!(r##"hello "\u{{007B}}""##); //~ useless_format
}
