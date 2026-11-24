fn issue16826() {
    match i == 1 {
        true => {
            println!("{i}")
        },
        false => (),
    }
}
