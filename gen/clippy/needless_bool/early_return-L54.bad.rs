fn multi_if_complex_previous_if(x: bool, y: bool) -> Result<bool, ()> {
    if x {
        return Ok(true);
    }
    Ok(false)
    //~^^^^ needless_bool
}
