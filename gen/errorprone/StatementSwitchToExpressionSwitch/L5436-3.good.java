class Test {
  public void foo(int val) {
    {
      int y;
      switch (val) {
        case 1 -> {
          y = 10;
        }
        case 2 -> y = 20;
        default -> {}
      }
    }
  }
}
