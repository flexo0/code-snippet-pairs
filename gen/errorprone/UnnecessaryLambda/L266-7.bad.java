class Test {
  void g() {
    Function<String, String> l = Test.notUpperCased;
    System.err.println(notUpperCased.apply("world"));
  }
}
