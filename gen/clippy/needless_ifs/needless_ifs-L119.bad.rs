fn issue_16845() {
    if true {}
    //~^ needless_ifs
    let () = if maybe_side_effect() {};
}
