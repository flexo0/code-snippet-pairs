pub fn issue13574() -> Option<()> {
    let _ = {
        if let Some(num) = A {
            //~^ needless_match
            Some(num)
        } else if let Some(num) = A {
            Some(num)
        } else if let Some(num) = A {
            Some(num)
        } else {
            None
        }
    };
}
