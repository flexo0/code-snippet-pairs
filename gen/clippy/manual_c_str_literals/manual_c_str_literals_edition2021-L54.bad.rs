fn main() {
    unsafe { CStr::from_ptr(b"foo\0".as_ptr().cast()) };
}
