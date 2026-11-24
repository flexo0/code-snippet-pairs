class Test {
  public void bar(int a) {
    while (true) {
      switch (a) {
        case 1 -> {
          return;
        }
        case 2 -> {
          return;
        }
        case 3 -> {
          return;
        }
        default -> {}
      }
    }
  }
}
