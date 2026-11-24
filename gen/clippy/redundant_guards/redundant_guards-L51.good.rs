fn main() {
    match x {
        Some(Some(1)) if true => ..,
        //~^ redundant_guards
        Some(Some(1)) => {
            //~^ redundant_guards
            println!("a");
            ..
        },
        Some(Some(1)) => ..,
        //~^ redundant_guards
        Some(Some(2)) => ..,
        //~^ redundant_guards
        Some(Some(2)) => ..,
        //~^ redundant_guards
        // Don't lint, since x is used in the body
        Some(x) if let Some(1) = x => {
            x;
            ..
        },
        _ => todo!(),
    };
}
