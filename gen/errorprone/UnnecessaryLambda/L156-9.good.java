class Test {
  void g() {
    Consumer<String> f = this::f;
    f("world");
  }
}
