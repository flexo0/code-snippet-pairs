pub struct DirectDefaultDefaultCall {
    v: Vec<i32>,
}

impl Default for DirectDefaultDefaultCall {
    //~^ derivable_impls
    fn default() -> Self {
        // When calling `Default::default()` in all fields, we know it is the same as deriving.
        Self { v: Default::default() }
    }
}
