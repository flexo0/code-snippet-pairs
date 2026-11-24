fn main() {
    unsafe {
        let _ = ptr.add(offset_usize);
        //~^ ptr_offset_with_cast
        let _ = ptr.offset(offset_isize as isize);
        let _ = ptr.offset(offset_u8 as isize);

        let _ = ptr.wrapping_add(offset_usize);
        //~^ ptr_offset_with_cast
        let _ = ptr.wrapping_offset(offset_isize as isize);
        let _ = ptr.wrapping_offset(offset_u8 as isize);

        let _ = S.offset(offset_usize as isize);
        let _ = S.wrapping_offset(offset_usize as isize);

        let _ = (&ptr).add(offset_usize);
        //~^ ptr_offset_with_cast
        let _ = (&ptr).wrapping_add(offset_usize);
        //~^ ptr_offset_with_cast
    }
}
