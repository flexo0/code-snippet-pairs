mod issue11465 {
    fn issue11465() {
        match c {
            Some(1) => {},
            //~^ redundant_guards
            Some(1) => {},
            //~^ redundant_guards
            Some(2) => {},
            //~^ redundant_guards
            Some(3) => {},
            //~^ redundant_guards
            _ => {},
        };
    }
}
