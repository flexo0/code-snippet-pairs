fn issue_8723() {
    val = match val.split_at(idx) {
        //~^ match_single_binding
        (pre, suf) => {
            println!("{pre}");
            suf
        },
    };
}
