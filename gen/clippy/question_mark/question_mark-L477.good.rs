fn issue_13417(foo: &mut StructWithOptionString) -> Option<String> {
    let x = foo.opt_x.as_ref()?;
}
