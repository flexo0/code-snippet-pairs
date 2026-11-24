fn main() {
    let _array_ptr_transmute = unsafe { transmute::<&[i32; 4], *const [i32; 4]>(array_ref) };
}
