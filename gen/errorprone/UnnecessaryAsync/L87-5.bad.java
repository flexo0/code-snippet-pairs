class Test {
  String test() {
    var ar = new AtomicReference<String>(null);
    ar.compareAndSet(null, "foo");
    return ar.get();
  }
}
