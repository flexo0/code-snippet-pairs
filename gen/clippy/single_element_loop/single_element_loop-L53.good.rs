fn main() {
    {
        let _ = 42;
        //~^ single_element_loop
        let _f = |n: u32| {
            for i in 0..n {
                if i > 10 {
                    dbg!(i);
                    break;
                }
            }
        };
    }
}
