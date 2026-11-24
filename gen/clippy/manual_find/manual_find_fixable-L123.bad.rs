fn tuple_with_ref(v: [(u8, &u8); 3]) -> Option<u8> {
    for (_, &x) in v {
        //~^ manual_find
        if x > 10 {
            return Some(x);
        }
    }
    None
}
