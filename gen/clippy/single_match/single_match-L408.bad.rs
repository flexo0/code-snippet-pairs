fn irrefutable_match() {
    match i {
        i => {
            let a = 1;
            let b = 2;
        },
        _ => {},
    }
}
