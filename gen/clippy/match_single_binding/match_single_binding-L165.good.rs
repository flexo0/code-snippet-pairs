fn issue_9725(r: Option<u32>) {
    let x = r;
    match x {
        Some(_) => {
            println!("Some");
        },
        None => {
            println!("None");
        },
    };
}
