fn indirect() -> Option<()> {
    let r = iter.next().unwrap();
    let l = iter.next().unwrap();

    let mut iter = "a.b.c".rsplitn(2, '.');
    //~^ manual_split_once
    let r = iter.next()?;
    let l = iter.next()?;
}
