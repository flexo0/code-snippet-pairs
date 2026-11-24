fn issue15568() {
    x.map(|x| unsafe { f(x) });
}
