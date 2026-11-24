fn issue6491() {
    while let Some(n) = it.next() {
        //~^ while_let_on_iterator
        let mut it = 1..40;
        while let Some(m) = it.next() {
            //~^ while_let_on_iterator
            if m % 10 == 0 {
                break;
            }
            println!("doing something with m: {}", m);
        }

        // Weird binding shouldn't change anything.
        let (mut it, _) = (1..40, 0);
        while let Some(m) = it.next() {
            //~^ while_let_on_iterator
            if m % 10 == 0 {
                break;
            }
            println!("doing something with m: {}", m);
        }

        // Used after the loop, needs &mut.
        let mut it = 1..40;
        while let Some(m) = it.next() {
            //~^ while_let_on_iterator
            if m % 10 == 0 {
                break;
            }
            println!("doing something with m: {}", m);
        }
        println!("next item {}", it.next().unwrap());

        println!("n still is {}", n);
    }
}
