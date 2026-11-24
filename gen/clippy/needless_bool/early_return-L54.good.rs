fn multi_if_complex_previous_if(x: bool, y: bool) -> Result<bool, ()> {
    Ok(x)
    //~^^^^ needless_bool
}
