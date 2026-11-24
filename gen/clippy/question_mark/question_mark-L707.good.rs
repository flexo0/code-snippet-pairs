fn issue16751(mut v: Option<usize>) -> Option<usize> {
    {
        let n = v?;
        if n > 10 { Some(42) } else { None }
    }
}
