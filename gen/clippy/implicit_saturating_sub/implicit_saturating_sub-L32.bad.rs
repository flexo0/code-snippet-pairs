fn main() {
    match end_8 {
        10 => {
            // Lint
            if u_8 > 0 {
                //~^ implicit_saturating_sub
                u_8 -= 1;
            }
        },
        11 => u_8 += 1,
        _ => u_8 = 0,
    }
}
