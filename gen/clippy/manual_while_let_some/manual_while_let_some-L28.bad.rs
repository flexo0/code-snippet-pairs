fn main() {
    while !val.numbers.is_empty() {
        let number = val.numbers.pop().unwrap();
        //~^ manual_while_let_some
    }
}
