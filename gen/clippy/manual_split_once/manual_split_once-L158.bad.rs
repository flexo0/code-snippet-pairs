fn _msrv_1_52() {
    let mut iter = "a.b.c".splitn(2, '.');
    //~^ manual_split_once
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
}
