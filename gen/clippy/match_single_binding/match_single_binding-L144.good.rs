fn issue_8723() {
    let (pre, suf) = val.split_at(idx);
    val = {
        println!("{pre}");
        suf
    };
}
