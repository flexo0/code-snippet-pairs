class Test {
  void test(Object o) {
    if (o instanceof Test) {
      test((Test) o);
      Test test = (Test) o;
      test(test);
    }
  }
}
