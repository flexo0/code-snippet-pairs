fn needless_bool_condition() -> bool {
    fn foo() -> bool {
        (unsafe { no(4) } & 1 != 0)
        //~^ needless_bool
    }
}
