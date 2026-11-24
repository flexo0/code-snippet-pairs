class Test {
  public void foo(Object o, boolean b) {
    do {
      if (o instanceof Integer) {
        continue;
      } else if (o instanceof String) {
        throw new AssertionError();
      } else if (o instanceof Object) {
        throw new AssertionError();
      }
      System.out.println("dead");
    } while (b);
  }
}
