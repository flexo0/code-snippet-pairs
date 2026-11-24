class Test {
  int test() {
    var ai = new AtomicInteger();
    ai.set(ai.get() + 1);
    return ai.get();
  }
}
