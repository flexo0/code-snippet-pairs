fn main() {
    let _ = Some(0).inspect(|&x| {
        //~^ manual_inspect
        println!("{x}");
    });
}
