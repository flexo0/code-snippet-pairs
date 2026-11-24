mod issue_5729 {
    impl<T: Foo + 'static> FooStorage for FooStorageImpl<T> {
        fn foo_cloned(&self) -> Arc<dyn Foo> {
            let clone = Arc::clone(&self.foo);
            clone
            //~^ let_and_return
        }
    }
}
