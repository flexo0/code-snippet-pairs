fn tuple_structs() {
    let _ = x.into_iter().map(|Foo2(foo, bar)| Foo2(foo, bar));
}
