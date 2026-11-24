fn irrefutable_match() {
    match i {
        i => {},
        _ => {},
    }
    //~^^^^ single_match

    match i {
        i => (),
        _ => (),
    }
    //~^^^^ single_match

    match CONST_I32 {
        CONST_I32 => println!(),
        _ => {},
    }
}
