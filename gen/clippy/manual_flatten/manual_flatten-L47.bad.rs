fn main() {
    for n in z {
        //~^ manual_flatten

        if let Some(m) = n {
            println!("{}", m);
        }
    }
}
