fn main() {
    let _ = if gen_opt().is_some() {
        //~^ redundant_pattern_matching
        1
    } else if gen_opt().is_none() {
        //~^ redundant_pattern_matching
        2
    } else {
        3
    };
}
