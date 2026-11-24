fn main() {
    let _ = match &s {
        //~^ manual_map
        Some(x) => Some({ if let Some(ref s) = s { (x.clone(), s) } else { panic!() } }),
        None => None,
    };
}
