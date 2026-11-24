fn issue15932() {
    let _: Option<&dyn std::fmt::Debug> = Some(0).as_ref().map(|x| x as _);
}
