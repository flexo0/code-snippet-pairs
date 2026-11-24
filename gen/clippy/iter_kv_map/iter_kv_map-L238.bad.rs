fn issue16742() {
    map.iter().flat_map(|(_, v)| v);
}
