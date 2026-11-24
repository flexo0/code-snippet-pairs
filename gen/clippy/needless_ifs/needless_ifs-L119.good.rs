fn issue_16845() {
    let () = if maybe_side_effect() {};
}
