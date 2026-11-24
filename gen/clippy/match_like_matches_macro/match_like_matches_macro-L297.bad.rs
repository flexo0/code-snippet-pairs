fn issue16015<T: 'static, U: 'static>() -> bool {
    if let _ = typeid!(U) { true } else { false }
}
