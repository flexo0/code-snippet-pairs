class Test {
  void m(int i) {
    if (i <= 0) {
      throw new IllegalArgumentException(String.format("expected > 0, got %s", i));
    }
  }
}
