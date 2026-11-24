fn issue15063(x: bool, y: bool) {
    if x && y {
        todo!()
    } else if x || y {
        z = true;
    } else {
        z = false;
    }
}
