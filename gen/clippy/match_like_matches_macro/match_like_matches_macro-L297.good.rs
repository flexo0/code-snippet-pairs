fn issue16015<T: 'static, U: 'static>() -> bool {
    matches!(typeid!(U), _)
    //~^ match_like_matches_macro
}
