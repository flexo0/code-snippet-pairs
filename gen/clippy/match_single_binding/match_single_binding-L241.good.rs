fn issue14634() {
    let id!(b) = dbg!(3);
    let id!(_a) = dbg!(b + 1);
}
