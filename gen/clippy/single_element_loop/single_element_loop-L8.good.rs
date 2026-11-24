fn main() {
    {
        let item = &item1;
        //~^ single_element_loop
        dbg!(item);
    }
}
