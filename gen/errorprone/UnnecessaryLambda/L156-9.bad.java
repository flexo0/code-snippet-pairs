class Test {
  void g() {
    Consumer<String> f = f();
    f().accept("world");
  }
}
