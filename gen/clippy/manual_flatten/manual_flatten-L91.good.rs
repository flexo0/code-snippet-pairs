fn main() {
    for n in vec_of_ref.iter().copied().flatten() {
        println!("{:?}", n);
    }
}
