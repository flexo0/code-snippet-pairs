fn main() {
    {
        // lint
        let _ans = !matches!(x, E::B(_) | E::C);
        //~^^^^^ match_like_matches_macro
    }
}
