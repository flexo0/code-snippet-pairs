mod issue11465 {
    fn issue11465() {
        match c {
            Some(ref x) if x == &1 => {},
            //~^ redundant_guards
            Some(ref x) if &1 == x => {},
            //~^ redundant_guards
            Some(ref x) if let &2 = x => {},
            //~^ redundant_guards
            Some(ref x) if matches!(x, &3) => {},
            //~^ redundant_guards
            _ => {},
        };
    }
}
