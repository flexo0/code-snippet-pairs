mod macros {
    fn option_with_macro(opt: Option<i32>) -> i32 {
        opt.map_or(mac!(42), |x| x)
        //~^ map_or_identity
    }
}
