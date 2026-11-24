fn main() {
    v.clone().iter().filter(|&i| (i == &NonCopy)).map(|i| i);
}
