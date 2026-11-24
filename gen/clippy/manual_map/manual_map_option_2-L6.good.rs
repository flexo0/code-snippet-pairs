fn main() {
    let _ = Some(0).map(|x| {
            let y = (String::new(), String::new());
            (x, y.0)
        });
}
