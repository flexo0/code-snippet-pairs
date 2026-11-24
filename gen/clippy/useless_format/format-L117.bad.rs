fn main() {
    literal(&format!("{}", lit)); //~ useless_format
    literal(&format!("{lit}")); //~ useless_format
}
