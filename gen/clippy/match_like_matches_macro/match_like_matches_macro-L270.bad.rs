fn issue15841(opt: Option<Option<Option<i32>>>, value: i32) {
    let _ = match opt {
        Some(first) if (if let Some(second) = first { true } else { todo!() }) => true,
        _ => false,
    };
}
