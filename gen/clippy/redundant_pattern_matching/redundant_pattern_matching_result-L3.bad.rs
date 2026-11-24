#![allow(
    clippy::if_same_then_else,
    clippy::match_like_matches_macro,
    clippy::needless_bool,
    clippy::needless_ifs,
    clippy::uninlined_format_args,
    clippy::unnecessary_wraps
)]

fn main() {
    let result: Result<usize, usize> = Err(5);
    if let Ok(_) = &result {}
    //~^ redundant_pattern_matching

    if let Ok(_) = Ok::<i32, i32>(42) {}
    //~^ redundant_pattern_matching

    if let Err(_) = Err::<i32, i32>(42) {}
    //~^ redundant_pattern_matching

    while let Ok(_) = Ok::<i32, i32>(10) {}
    //~^ redundant_pattern_matching

    while let Err(_) = Ok::<i32, i32>(10) {}
    //~^ redundant_pattern_matching

    if Ok::<i32, i32>(42).is_ok() {}

    if Err::<i32, i32>(42).is_err() {}

    if let Ok(x) = Ok::<i32, i32>(42) {
        println!("{}", x);
    }

    match Ok::<i32, i32>(42) {
        //~^ redundant_pattern_matching
        Ok(_) => true,
        Err(_) => false,
    };

    match Ok::<i32, i32>(42) {
        //~^ redundant_pattern_matching
        Ok(_) => false,
        Err(_) => true,
    };

    match Err::<i32, i32>(42) {
        //~^ redundant_pattern_matching
        Ok(_) => false,
        Err(_) => true,
    };

    match Err::<i32, i32>(42) {
        //~^ redundant_pattern_matching
        Ok(_) => true,
        Err(_) => false,
    };

    let _ = if let Ok(_) = Ok::<usize, ()>(4) { true } else { false };
    //~^ redundant_pattern_matching

    issue5504();
    issue6067();
    issue6065();
    issue10726();
    issue10803();

    let _ = if let Ok(_) = gen_res() {
        //~^ redundant_pattern_matching
        1
    } else if let Err(_) = gen_res() {
        //~^ redundant_pattern_matching
        2
    } else {
        3
    };
}
