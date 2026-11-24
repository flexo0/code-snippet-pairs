class Test {
  int test() {
    var ai = new AtomicInteger();
    ai.set(1);
    return ai.get();
  }
}
