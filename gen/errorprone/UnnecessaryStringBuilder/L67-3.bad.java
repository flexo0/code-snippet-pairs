class Test {
  void f(String hello) {
    String a = new StringBuilder().append(hello).append("world").toString();
    StringBuilder b = new StringBuilder().append(hello).append("world");
  }
}
