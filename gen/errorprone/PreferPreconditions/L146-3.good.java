class Test {
  void m(int i) {
    checkArgument(i > 0, "expected > 0, got %s", i);
  }
}
