fn main() {
    let _ = Some(0).map(|x| {
        //~^ manual_inspect
        println!("{x}");
        x
    });
}
