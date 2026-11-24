class Test {
  public void foo(Object o, int k, boolean flag) {
    lbl:
    switch (k) {
      default:
        throw new AssertionError();
      case 1:
        if (o instanceof Integer) {
          throw new AssertionError();
        } else if (o instanceof String) {
          throw new AssertionError();
        } else if (o instanceof Object) {
          throw new AssertionError();
        }
        while (flag) {
          break lbl;
        }
        System.out.println("dead");
    }
    System.out.println("afterSwitch");
  }
}
