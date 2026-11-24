fn main() {
    for n in z {
        //~^ manual_flatten

        if let Ok(n) = n {
            println!("{}", n);
        }
    }
}
