fn issue14634() {
    match dbg!(3) {
        id!(a) => println!("found {a}"),
    }
}
