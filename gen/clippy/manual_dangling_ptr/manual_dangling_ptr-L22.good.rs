fn main() {
    foo(std::ptr::dangling(), std::ptr::dangling_mut());
}
