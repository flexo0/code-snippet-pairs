pub struct RepeatDefault1 {
    a: [i8; 32],
}

impl Default for RepeatDefault1 {
    //~^ derivable_impls
    fn default() -> Self {
        RepeatDefault1 { a: [0; 32] }
    }
}
