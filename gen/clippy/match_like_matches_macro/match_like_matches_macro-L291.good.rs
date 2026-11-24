fn issue16015<T: 'static, U: 'static>() -> bool {
    matches!(typeid!(T), _);
}
