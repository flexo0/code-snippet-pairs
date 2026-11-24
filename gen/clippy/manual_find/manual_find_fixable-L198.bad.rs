fn as_closure() {
    let f = |arr: Vec<i32>| -> Option<i32> {
        for x in arr {
        //~^ manual_find
            if x < 1 {
                return Some(x);
            }
        }
        None
    };
}
