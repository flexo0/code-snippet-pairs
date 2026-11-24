fn unnecessary_fold() {
    let _: i32 = (0..3).fold(0, Add::add);
}
