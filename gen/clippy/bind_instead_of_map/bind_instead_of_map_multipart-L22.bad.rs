fn hard_example() {
    Some("42").and_then(|s| {
        //~^ bind_instead_of_map
        if {
            if s == "43" {
                return Some(43);
            }
            s == "42"
        } {
            return Some(45);
        }
        match s.len() {
            10 => Some(2),
            20 => {
                if foo() {
                    return {
                        if foo() {
                            return Some(20);
                        }
                        println!("foo");
                        Some(3)
                    };
                }
                Some(20)
            },
            40 => Some(30),
            _ => Some(1),
        }
    });
}
