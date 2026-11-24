class Test {
  void f(String hello) {
    System.err.println(new StringBuilder().append(hello).append("world"));
    System.err.println(new StringBuilder(hello).append("world"));
    System.err.println(new StringBuilder(10).append(hello).append("world"));
    System.err.println(new StringBuilder(hello).append("world").toString());
    System.err.println(new StringBuilder().toString());
  }
}
