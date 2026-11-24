fn main() {
    let _: &str = "hello"; //~ useless_format
    let _: &str = "{}"; //~ useless_format
    let _: &str = r"hello"; //~ useless_format
    let _: &str = r#"world"#; //~ useless_format
    let _: &str = r##"foo"##; //~ useless_format
    let _: &str = r##"hello "\u{007B}""##; //~ useless_format
}
