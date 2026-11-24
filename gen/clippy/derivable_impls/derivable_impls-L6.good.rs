struct FooDefault<'a> {
    a: bool,
    b: i32,
    c: u64,
    d: Vec<i32>,
    e: FooND1,
    f: FooND2,
    g: HashMap<i32, i32>,
    h: (i32, Vec<i32>),
    i: [Vec<i32>; 3],
    j: [i32; 5],
    k: Option<i32>,
    l: &'a [i32],
}
