fn issue11141() {
    let _ = (if true { 30 } else { 17 }) | if true { 2 } else { 3 } | if true { 10 } else { 1 };
}
