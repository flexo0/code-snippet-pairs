fn main() {
    match a {
        //~^ match_single_binding
        _ => {
            let x = 29;
            println!("x has a value of {x}");
        },
    }
    // Lint
    match a {
        //~^ match_single_binding
        _ => {
            let e = 5 * a;
            if e >= 5 {
                println!("e is superior to 5");
            }
        },
    }
}
