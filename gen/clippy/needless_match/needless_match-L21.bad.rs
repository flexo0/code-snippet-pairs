fn useless_match() {
    let _: &str = match s {
        //~^ needless_match
        "a" => "a",
        "b" => "b",
        s => s,
    };
}
