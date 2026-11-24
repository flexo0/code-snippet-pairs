fn main() {
    v.clone().iter().filter_map(|i| (i == &NonCopy).then(|| i));
}
