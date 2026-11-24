fn main() {
    {
        // lint
        let _ans = match x {
            E::A(_) => true,
            E::B(_) => true,
            _ => false,
        };
        //~^^^^^ match_like_matches_macro
    }
}
