class Test {
  public void bar(Object o, boolean b) {
    do {
      if (o instanceof Integer) {
        throw new AssertionError();
      } else if (o instanceof String) {
        throw new AssertionError();
      } else if (o instanceof Long) {
        throw new AssertionError();
      }
      continue;
    } while (b);
  }
}
