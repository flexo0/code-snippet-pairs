fn issue9956() {
    let a = (|| || || 123)();
}
