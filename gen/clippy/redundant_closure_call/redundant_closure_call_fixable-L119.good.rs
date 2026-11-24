mod issue11707 {
    fn demo_with_captures() {
        spawn_on(async move { drop(format!("hello, {name}")) });
    }
}
