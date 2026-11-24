fn main() {
    for item in &[item1] {
        //~^ single_element_loop
        dbg!(item);
    }
}
