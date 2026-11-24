async fn async_fn() -> Result<(), ()> {
    if true {
        return Err(())?;
        //~^ needless_return_with_question_mark
    }
}
