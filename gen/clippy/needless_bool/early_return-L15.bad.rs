fn bare(x: bool) -> bool {
    if x {
        return true;
    }
    false
    //~^^^^ needless_bool
}
