fn irrefutable_match() {
    match DATA {
        DATA => println!(),
        _ => {},
    }
}
