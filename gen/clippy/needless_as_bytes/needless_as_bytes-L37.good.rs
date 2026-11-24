fn main() {
    if s.is_empty() {
        //~^ needless_as_bytes

        println!("len = {}", s.len());
        //~^ needless_as_bytes
    }
}
