fn main() {
    match Some(11) {
        //~^ manual_filter
        // Lint, statement is preserved by `.filter`
        Some(x) => {
            if {
                println!("foo");
                x > 10 && x < 100
            } {
                Some(x)
            } else {
                None
            }
        },
        None => None,
    };
}
