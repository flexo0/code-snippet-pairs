fn field() {
    let temp = bar.a;
    //~^ manual_swap
    bar.a = bar.b;
    bar.b = temp;
}
