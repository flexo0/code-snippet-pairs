fn issue8010() {
    let _x = iter
        .clone()
        .filter(|x| matches!(x, Enum::A(_)))
        //~^ manual_filter_map
        .map(|x| if let Enum::A(s) = x { s } else { unreachable!() });
}
