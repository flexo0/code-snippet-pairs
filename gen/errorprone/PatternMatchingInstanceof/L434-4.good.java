class Test {
  void test(Object o) {
    if (o instanceof Long l) {
      test(l);
      test(l.hashCode());
    }
  }
}
