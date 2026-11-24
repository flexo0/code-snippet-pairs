class Test {
  public int[] foo() {
    myLabel:
    {
      String foo;
      switch (z) {
        case 1 -> {
          foo = "hello";
        }
        case 2 -> foo = "there";
      }
    }
  }
}
