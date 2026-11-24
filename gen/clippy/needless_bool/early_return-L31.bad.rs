fn option_wrapped(x: bool) -> Option<bool> {
    if x {
        return Some(false);
    }
    Some(true)
    //~^^^^ needless_bool
}
