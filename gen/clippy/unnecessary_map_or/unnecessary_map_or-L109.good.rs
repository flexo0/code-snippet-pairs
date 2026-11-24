fn with_refs(o: &mut Option<u32>) -> bool {
    o.is_none_or(|n| n > 5) || (o as &Option<u32>).is_none_or(|n| n < 5)
    //~^ unnecessary_map_or
    //~| unnecessary_map_or
}
