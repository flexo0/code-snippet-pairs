fn main() {
    match x {
        Some(x) if matches!(x, Some(1) if true) => ..,
        //~^ redundant_guards
        Some(x) if matches!(x, Some(1)) => {
            //~^ redundant_guards
            println!("a");
            ..
        },
        Some(x) if let Some(1) = x => ..,
        //~^ redundant_guards
        Some(x) if x == Some(2) => ..,
        //~^ redundant_guards
        Some(x) if Some(2) == x => ..,
        //~^ redundant_guards
        // Don't lint, since x is used in the body
        Some(x) if let Some(1) = x => {
            x;
            ..
        },
        _ => todo!(),
    };
}
