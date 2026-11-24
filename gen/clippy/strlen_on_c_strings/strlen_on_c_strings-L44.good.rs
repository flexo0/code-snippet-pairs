fn main() {
    let _ = unsafe {
        f(cstr).count_bytes()
        //~^ ERROR: using `libc::strlen` on a `CStr` value
    };
}
