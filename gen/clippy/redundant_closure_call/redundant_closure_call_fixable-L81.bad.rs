fn issue9956() {
    let a = (|| echo!((|| 123)))()();
}
