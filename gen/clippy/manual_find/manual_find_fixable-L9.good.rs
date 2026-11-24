fn lookup(n: u32) -> Option<u32> {
    ARRAY.iter().find(|&&v| v == n).copied()
}
