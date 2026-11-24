fn test_inner() -> i32 {
    if true {
        let x = 5;
        x
        //~^ let_and_return
    } else {
        0
    }
}
