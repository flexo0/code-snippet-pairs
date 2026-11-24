fn main() {
    let _ = TupleStruct {
        //~^ init_numbered_fields
        0: 1u32,
        1: 42,
        2: 23u8,
    };
}
