fn main() {
    let _array_ptr_transmute = unsafe { array_ref as *const [i32; 4] };
}
