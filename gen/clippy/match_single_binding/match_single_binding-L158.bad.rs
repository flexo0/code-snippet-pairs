fn issue_9575() {
    let _ = || match side_effects() {
        //~^ match_single_binding
        _ => println!("Needs curlies"),
    };
}
