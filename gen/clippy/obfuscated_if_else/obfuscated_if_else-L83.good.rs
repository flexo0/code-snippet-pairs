fn issue11141() {
    let _ = *if true { &42 } else { &17 };
}
