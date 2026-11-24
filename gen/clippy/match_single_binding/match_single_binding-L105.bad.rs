fn main() {
    let _ = v
        .iter()
        .map(|i| match i.unwrap() {
            //~^ match_single_binding
            unwrapped => unwrapped,
        })
        .collect::<Vec<u8>>();
}
