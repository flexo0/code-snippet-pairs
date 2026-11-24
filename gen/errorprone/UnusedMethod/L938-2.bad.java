class Test {
  private interface Frobnicator {
    // BUG: Diagnostic contains:
    int frobnicate();
  }
}
