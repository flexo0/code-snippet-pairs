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
    if result.is_ok() {}
    //~^ redundant_pattern_matching

    if Ok::<i32, i32>(42).is_ok() {}
    //~^ redundant_pattern_matching

    if Err::<i32, i32>(42).is_err() {}
    //~^ redundant_pattern_matching

    while Ok::<i32, i32>(10).is_ok() {}
    //~^ redundant_pattern_matching

    while Ok::<i32, i32>(10).is_err() {}
    //~^ redundant_pattern_matching

    if Ok::<i32, i32>(42).is_ok() {}

    if Err::<i32, i32>(42).is_err() {}

    if let Ok(x) = Ok::<i32, i32>(42) {
        println!("{}", x);
    }

    Ok::<i32, i32>(42).is_ok();

    Ok::<i32, i32>(42).is_err();

    Err::<i32, i32>(42).is_err();

    Err::<i32, i32>(42).is_ok();

    let _ = if Ok::<usize, ()>(4).is_ok() { true } else { false };
    //~^ redundant_pattern_matching

    issue5504();
    issue6067();
    issue6065();
    issue10726();
    issue10803();

    let _ = if gen_res().is_ok() {
        //~^ redundant_pattern_matching
        1
    } else if gen_res().is_err() {
        //~^ redundant_pattern_matching
        2
    } else {
        3
    };
}
