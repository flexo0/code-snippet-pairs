fn issue14634() {
    let id!(_a) = match dbg!(3) {
        id!(b) => dbg!(b + 1),
    };
}
