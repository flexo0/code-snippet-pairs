fn issue_9725(r: Option<u32>) {
    match r {
        //~^ match_single_binding
        x => match x {
            Some(_) => {
                println!("Some");
            },
            None => {
                println!("None");
            },
        },
    };
}
