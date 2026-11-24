class Test {
  public void foo(Object o, int x) {
    l:
    while (true) {
      switch (x) {
        default:
          break l;
        case 1:
          if (o instanceof Integer) {
            throw new AssertionError();
          } else if (o instanceof String) {
            throw new AssertionError();
          } else if (o instanceof Object) {
            throw new AssertionError();
          }
          System.out.println("remove me");
      }
    }
  }
}
