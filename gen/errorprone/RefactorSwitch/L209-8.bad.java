class Test {
  public void foo(int ui) {
    switch (ui) {
      case 1 -> x = new Foo();
      case 2 -> x = (x.field = new Foo());
      default -> x = new Foo();
    }
  }
}
