class Test {
  void test(Object o) {
    if (!(o instanceof Test)) {
      return;
    }
    Test test = (Test) o;
  }
}
