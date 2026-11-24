fn indirect() -> Option<()> {
    let (l, r) = "a.b.c".split_once('.')?;
}
