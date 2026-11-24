fn main() {
    while !numbers.is_empty() {
        let number = numbers.pop().unwrap();
        //~^ manual_while_let_some
    }
}
