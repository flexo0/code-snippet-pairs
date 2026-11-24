fn main() {
    let x = ""; //~ useless_concat
    let x = "c"; //~ useless_concat
    let x = "\""; //~ useless_concat
    let x = "true"; //~ useless_concat
    let x = "1"; //~ useless_concat
    let x = "1.0000"; //~ useless_concat
    let x = "1"; //~ useless_concat
    let x = "1"; //~ useless_concat
    let x = "1.0000"; //~ useless_concat
    let x = "1.0000"; //~ useless_concat
    let x = "a😀\n"; //~ useless_concat
    let x = "a"; //~ useless_concat
    let x = "1"; //~ useless_concat
    println!("b: {}", "a"); //~ useless_concat
}
