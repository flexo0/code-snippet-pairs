class Test {
  void m(double d) {
    if (d < 0.0) {
      throw new IllegalArgumentException();
    }
  }
}
