fn main() {
    while !numbers.is_empty() {
        accept_i32(numbers.pop().unwrap());
        //~^ manual_while_let_some
    }
}
