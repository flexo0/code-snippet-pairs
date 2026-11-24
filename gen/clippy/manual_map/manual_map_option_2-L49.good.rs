fn main() {
    let _ = s.as_ref().map(|x| { if let Some(ref s) = s { (x.clone(), s) } else { panic!() } });
}
