mod issue_5729 {
    impl<T: Foo + 'static> FooStorage for FooStorageImpl<T> {
        fn foo_cloned(&self) -> Arc<dyn Foo> {
            (Arc::clone(&self.foo)) as _
            //~^ let_and_return
        }
    }
}
