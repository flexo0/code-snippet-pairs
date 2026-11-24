fn main() {
    while let Some(..) = it.next() {
        //~^ while_let_on_iterator
        println!("test");
    }
}
