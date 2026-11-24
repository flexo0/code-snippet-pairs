fn issue_9575() {
    let _ = || {
        side_effects();
        println!("Needs curlies")
    };
}
