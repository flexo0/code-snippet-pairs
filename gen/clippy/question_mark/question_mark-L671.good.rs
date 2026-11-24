fn issue16654() -> Result<(), i32> {
    _ = [{ result?; }];
}
