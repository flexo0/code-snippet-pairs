fn issue8010() {
    let _x = iter.clone().filter(|x| matches!(x, Enum::A(_))).map(|x| match x {
        //~^ manual_filter_map
        Enum::A(s) => s,
        _ => unreachable!(),
    });
}
