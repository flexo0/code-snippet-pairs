fn main() {
    let _ = v
        .iter()
        .map(|i| {
            let unwrapped = i.unwrap();
            unwrapped
        })
        .collect::<Vec<u8>>();
}
