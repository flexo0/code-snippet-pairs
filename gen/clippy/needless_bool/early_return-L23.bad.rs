fn bare_negated(x: bool) -> bool {
    if x {
        return false;
    }
    true
    //~^^^^ needless_bool
}
