fn standard() {
    let mut v1: Vec<usize> = vec![];
    //~^ reserve_after_initialization
    v1.reserve(10);
}
