fn main() {
    let x = concat!(); //~ useless_concat
    let x = concat!('c'); //~ useless_concat
    let x = concat!('"'); //~ useless_concat
    let x = concat!(true); //~ useless_concat
    let x = concat!(1f32); //~ useless_concat
    let x = concat!(1.0000f32); //~ useless_concat
    let x = concat!(1_f32); //~ useless_concat
    let x = concat!(1_); //~ useless_concat
    let x = concat!(1.0000_f32); //~ useless_concat
    let x = concat!(1.0000_); //~ useless_concat
    let x = concat!("a\u{1f600}\n"); //~ useless_concat
    let x = concat!(r##"a"##); //~ useless_concat
    let x = concat!(1); //~ useless_concat
    println!("b: {}", concat!("a")); //~ useless_concat
}
