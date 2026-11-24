fn main() {
    let _ = if let Some(_) = gen_opt() {
        //~^ redundant_pattern_matching
        1
    } else if let None = gen_opt() {
        //~^ redundant_pattern_matching
        2
    } else {
        3
    };
}
