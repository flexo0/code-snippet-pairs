fn tuple_with_ref(v: [(u8, &u8); 3]) -> Option<u8> {
    v.into_iter().map(|(_, &x)| x).find(|&x| x > 10)
}
