fn issue9956() {
    let a = (|| || || || async || 1)()()()()();
}
