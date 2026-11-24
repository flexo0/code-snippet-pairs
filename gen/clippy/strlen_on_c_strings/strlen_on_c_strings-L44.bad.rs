fn main() {
    let _ = unsafe {
        strlen(f(cstr).as_ptr())
        //~^ ERROR: using `libc::strlen` on a `CStr` value
    };
}
