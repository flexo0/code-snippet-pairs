struct TupleDefault(bool, i32, u64);

impl std::default::Default for TupleDefault {
    //~^ derivable_impls
    fn default() -> Self {
        Self(false, 0, 0u64)
    }
}
