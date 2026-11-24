mod macros {
    fn option_with_macro(opt: Option<i32>) -> i32 {
        opt.unwrap_or(mac!(42))
        //~^ map_or_identity
    }
}
