fn main() {
    CStr::from_bytes_with_nul(b"foo\0").unwrap();
}
