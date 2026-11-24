fn recv_requiring_parens() {
    let _ = match !S {
        //~^ match_as_ref
        None => None,
        Some(ref v) => Some(v),
    };
}
