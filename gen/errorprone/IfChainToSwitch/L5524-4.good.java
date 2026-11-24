class Test {
  public void foo(Object o, int k) {
    switch (k) {
      case 1:
        switch (o) {
          case Integer _ -> throw new AssertionError();
          case String _ -> throw new AssertionError();
          case Object _ -> throw new AssertionError();
          case null -> {}
        }
    }
  }
}
