pub enum SimpleEnum {
    Foo,
    Bar,
}

impl Default for SimpleEnum {
    //~^ derivable_impls
    fn default() -> Self {
        SimpleEnum::Bar
    }
}
