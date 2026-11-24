fn needless_bool_condition() -> bool {
    fn foo() -> bool {
        if unsafe { no(4) } & 1 != 0 { true } else { false }
    }
}
