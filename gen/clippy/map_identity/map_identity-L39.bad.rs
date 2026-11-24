fn issue7189() {
    let _ = x.clone().map(|(x, y)| {
        //~^ map_identity
        return (x, y);
    });
    let _ = x.clone().map(|(x, y)| return (x, y));
}
