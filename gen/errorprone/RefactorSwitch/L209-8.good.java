class Test {
  public void foo(int ui) {
    x =
        switch (ui) {
          case 1 -> new Foo();
          case 2 -> (x.field = new Foo());
          default -> new Foo();
        };
  }
}
