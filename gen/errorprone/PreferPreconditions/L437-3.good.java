class Test {
  void m(String s) {
    checkArgument(!s.isEmpty(), "got %s", s.length());
  }
}
