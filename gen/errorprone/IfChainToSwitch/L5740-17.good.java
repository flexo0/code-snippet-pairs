class Test {
  public void bar(Object o, boolean b) {
    do {
      switch (o) {
        case Integer _ -> throw new AssertionError();
        case String _ -> throw new AssertionError();
        case Long _ -> throw new AssertionError();
        default -> {
          continue;
        }
      }

    } while (b);
  }
}
