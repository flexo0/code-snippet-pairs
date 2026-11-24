fn func() -> Option<i32> {
    if f().is_none() {
        //~^ question_mark
        return None;
    }
}
