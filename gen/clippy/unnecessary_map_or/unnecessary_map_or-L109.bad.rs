fn with_refs(o: &mut Option<u32>) -> bool {
    o.map_or(true, |n| n > 5) || (o as &Option<u32>).map_or(true, |n| n < 5)
    //~^ unnecessary_map_or
    //~| unnecessary_map_or
}
