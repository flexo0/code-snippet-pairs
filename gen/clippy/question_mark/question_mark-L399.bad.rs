fn issue6828_nested_body() -> Option<u32> {
    try {
        fn f2(a: Option<i32>) -> Option<i32> {
            if a.is_none() {
                //~^ question_mark
                return None;
                // do lint here, the outer `try` is not relevant here
                // https://github.com/rust-lang/rust-clippy/pull/11001#issuecomment-1610636867
            }
            Some(32)
        }
        123
    }
}
