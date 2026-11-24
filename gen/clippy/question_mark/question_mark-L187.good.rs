fn func() -> Option<i32> {
    {
        let val = f()?;
        println!("{val}");
        val
    };
}
