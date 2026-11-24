struct Y(u32);
impl Default for Y {
    //~^ derivable_impls
    fn default() -> Self {
        Self(mac!())
    }
}
