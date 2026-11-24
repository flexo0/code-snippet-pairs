fn useless_match() {
    let _: i32 = match i {
        //~^ needless_match
        0 => 0,
        1 => 1,
        2 => 2,
        _ => i,
    };
}
