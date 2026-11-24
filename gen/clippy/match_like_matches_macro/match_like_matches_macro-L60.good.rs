fn main() {
    {
        // lint
        let _ans = matches!(x, E::A(_) | E::B(_));
        //~^^^^^ match_like_matches_macro
    }
}
