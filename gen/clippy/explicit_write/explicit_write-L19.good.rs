fn main() {
    {
        use std::io::Write;
        print!("test");
        //~^ explicit_write
        eprint!("test");
        //~^ explicit_write
        println!("test");
        //~^ explicit_write
        eprintln!("test");
        //~^ explicit_write
        print!("test");
        //~^ explicit_write
        eprint!("test");
        //~^ explicit_write

        // including newlines
        println!("test\ntest");
        //~^ explicit_write
        eprintln!("test\ntest");
        //~^ explicit_write

        let value = 1;
        eprintln!("with {value}");
        //~^ explicit_write
        eprintln!("with {} {}", 2, value);
        //~^ explicit_write
        eprintln!("with {value}");
        //~^ explicit_write
        eprintln!("macro arg {}", one!());
        //~^ explicit_write
        let width = 2;
        eprintln!("{value:w$}", w = width);
        //~^ explicit_write
    }
}
