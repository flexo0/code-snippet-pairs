fn issue12846() {
    let _x = if a && b { true } else { false } as u8;
}
