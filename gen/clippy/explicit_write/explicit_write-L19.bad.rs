fn main() {
    {
        use std::io::Write;
        write!(std::io::stdout(), "test").unwrap();
        //~^ explicit_write
        write!(std::io::stderr(), "test").unwrap();
        //~^ explicit_write
        writeln!(std::io::stdout(), "test").unwrap();
        //~^ explicit_write
        writeln!(std::io::stderr(), "test").unwrap();
        //~^ explicit_write
        std::io::stdout().write_fmt(format_args!("test")).unwrap();
        //~^ explicit_write
        std::io::stderr().write_fmt(format_args!("test")).unwrap();
        //~^ explicit_write

        // including newlines
        writeln!(std::io::stdout(), "test\ntest").unwrap();
        //~^ explicit_write
        writeln!(std::io::stderr(), "test\ntest").unwrap();
        //~^ explicit_write

        let value = 1;
        writeln!(std::io::stderr(), "with {value}").unwrap();
        //~^ explicit_write
        writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
        //~^ explicit_write
        writeln!(std::io::stderr(), "with {value}").unwrap();
        //~^ explicit_write
        writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
        //~^ explicit_write
        let width = 2;
        writeln!(std::io::stderr(), "{value:w$}", w = width).unwrap();
        //~^ explicit_write
    }
}
