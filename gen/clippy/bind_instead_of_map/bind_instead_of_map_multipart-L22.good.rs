fn hard_example() {
    Some("42").map(|s| {
        //~^ bind_instead_of_map
        if {
            if s == "43" {
                return 43;
            }
            s == "42"
        } {
            return 45;
        }
        match s.len() {
            10 => 2,
            20 => {
                if foo() {
                    return {
                        if foo() {
                            return 20;
                        }
                        println!("foo");
                        3
                    };
                }
                20
            },
            40 => 30,
            _ => 1,
        }
    });
}
