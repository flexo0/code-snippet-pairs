fn issue15063(x: bool, y: bool) {
    if x && y {
        todo!()
    } else { z = x || y; }
}
