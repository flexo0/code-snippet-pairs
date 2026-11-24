class Test {
  void test(Object o) {
    if (o instanceof Test test) {
      test(test);

      test(test);
    }
  }
}
