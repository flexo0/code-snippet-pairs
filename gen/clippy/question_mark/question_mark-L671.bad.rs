fn issue16654() -> Result<(), i32> {
    _ = [if let Err(err) = result {
        //~^ question_mark
        return Err(err);
    }];
}
