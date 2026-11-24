class Test {
  void m(String s) {
    if (s.isEmpty()) {
      throw new IllegalArgumentException(String.format("got %s", s.length()));
    }
  }
}
