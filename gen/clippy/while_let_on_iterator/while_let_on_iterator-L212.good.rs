fn issue6491() {
    for n in it {
        //~^ while_let_on_iterator
        let mut it = 1..40;
        for m in it {
            //~^ while_let_on_iterator
            if m % 10 == 0 {
                break;
            }
            println!("doing something with m: {}", m);
        }

        // Weird binding shouldn't change anything.
        let (mut it, _) = (1..40, 0);
        for m in it {
            //~^ while_let_on_iterator
            if m % 10 == 0 {
                break;
            }
            println!("doing something with m: {}", m);
        }

        // Used after the loop, needs &mut.
        let mut it = 1..40;
        for m in it.by_ref() {
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
