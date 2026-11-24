fn main() {
    if u_32 < 42 {
        println!("brace yourself!");
    } else if u_32 < u32::MAX {
        //~^ implicit_saturating_add
        u_32 += 1;
    }
}
