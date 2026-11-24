fn issue8010() {
    let _x = iter
        .clone()
        .filter_map(|x| match x { Enum::A(s) => Some(s), _ => None });
}
