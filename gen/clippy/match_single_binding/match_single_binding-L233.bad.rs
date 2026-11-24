fn issue14634() {
    match dbg!(3) {
        _ => println!("here"),
    }
}
