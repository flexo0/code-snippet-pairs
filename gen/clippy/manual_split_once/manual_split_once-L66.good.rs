fn indirect() -> Option<()> {
    let (l, r) = "a.b.c".rsplit_once('.')?;
}
