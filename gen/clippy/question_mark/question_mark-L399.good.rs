fn issue6828_nested_body() -> Option<u32> {
    try {
        fn f2(a: Option<i32>) -> Option<i32> {
            a?;
            Some(32)
        }
        123
    }
}
