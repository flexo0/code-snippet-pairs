fn issue16015<T: 'static, U: 'static>() -> bool {
    match typeid!(T) {
        _ => true,
        _ => false,
    };
}
