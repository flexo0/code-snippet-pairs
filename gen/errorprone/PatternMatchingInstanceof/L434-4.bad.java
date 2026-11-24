class Test {
  void test(Object o) {
    if (o instanceof Long) {
      test((Long) o);
      test(((Long) o).hashCode());
    }
  }
}
