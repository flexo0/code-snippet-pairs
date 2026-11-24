fn main() {
    for item in [item1].iter() {
        //~^ single_element_loop
        dbg!(item);
    }
}
