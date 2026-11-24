fn main() {
    while let Some(element) = numbers.pop() {
        accept_i32(element);
        //~^ manual_while_let_some
    }
}
