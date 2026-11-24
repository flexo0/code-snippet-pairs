fn main() {
    for item in [0..5].iter_mut() {
        //~^ single_element_loop
        dbg!(item);
    }
}
