fn issue_13417(foo: &mut StructWithOptionString) -> Option<String> {
    let Some(ref x) = foo.opt_x else {
        return None;
    };
}
