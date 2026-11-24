fn main() {
    if s.bytes().is_empty() {
        //~^ needless_as_bytes

        println!("len = {}", s.bytes().len());
        //~^ needless_as_bytes
    }
}
