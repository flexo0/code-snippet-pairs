fn main() {
    let _: Option<i32> = opt.map_or(None, |x| Some(x + 1));
}
