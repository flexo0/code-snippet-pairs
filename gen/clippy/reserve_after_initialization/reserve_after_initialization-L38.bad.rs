fn assign_expression() {
    v5 = Vec::new();
    //~^ reserve_after_initialization
    v5.reserve(10);
}
