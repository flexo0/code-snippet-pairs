fn main() {
    for item in 0..5 {
        //~^ single_element_loop
        dbg!(item);
    }
}
