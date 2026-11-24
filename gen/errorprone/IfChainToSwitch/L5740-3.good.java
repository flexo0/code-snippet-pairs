class Test {
  public void foo(Object o, boolean b) {
    do {
      switch (o) {
        case Integer _ -> {
          continue;
        }
        case String _ -> throw new AssertionError();
        case Object _ -> throw new AssertionError();
      }
    } while (b);
  }
}
