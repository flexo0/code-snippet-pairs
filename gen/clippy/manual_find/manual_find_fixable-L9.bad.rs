fn lookup(n: u32) -> Option<u32> {
    for &v in ARRAY {
        //~^ manual_find
        if v == n {
            return Some(v);
        }
    }
    None
}
