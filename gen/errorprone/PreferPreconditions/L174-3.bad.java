class Test {
  void m(int i) {
    if (i != 42) {
      throw new IllegalStateException();
    }
  }
}
