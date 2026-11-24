class Test {
  private Supplier<String> f() {
    return () -> "hello ";
  }
}
