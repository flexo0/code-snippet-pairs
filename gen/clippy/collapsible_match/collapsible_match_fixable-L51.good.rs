fn issue16716(n: u32) {
    match n {
        0 | 1 if false => { println!("hello world"); },
        //~^ collapsible_match
        _ => (),
    }
}
