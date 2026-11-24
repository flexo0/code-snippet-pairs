fn main() {
    {
        // lint
        let _ans = match x {
            E::B(_) => false,
            E::C => false,
            _ => true,
        };
        //~^^^^^ match_like_matches_macro
    }
}
