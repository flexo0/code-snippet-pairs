fn issue9956() {
    let a = (|| echo!(|| echo!(|| 1)))()()();
}
