fn test_inner() -> i32 {
    if true {
        
        5
        //~^ let_and_return
    } else {
        0
    }
}
