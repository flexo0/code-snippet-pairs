fn main() {
    Some(11).filter(|&x| {
                println!("foo");
                x > 10 && x < 100
            });
}
