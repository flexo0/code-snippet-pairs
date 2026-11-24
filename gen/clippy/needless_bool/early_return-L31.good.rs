fn option_wrapped(x: bool) -> Option<bool> {
    Some(!x)
    //~^^^^ needless_bool
}
