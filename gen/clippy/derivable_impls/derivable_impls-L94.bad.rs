struct StrDefault<'a>(&'a str);

impl Default for StrDefault<'_> {
    //~^ derivable_impls
    fn default() -> Self {
        Self("")
    }
}
