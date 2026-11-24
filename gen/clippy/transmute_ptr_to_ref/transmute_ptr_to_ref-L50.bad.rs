fn issue1231() {
    unsafe { std::mem::transmute::<_, Bar>(raw) };
}
