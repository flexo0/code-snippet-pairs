fn main() {
    for n in vec_of_ref {
        //~^ manual_flatten

        if let Some(n) = n {
            println!("{:?}", n);
        }
    }
}
