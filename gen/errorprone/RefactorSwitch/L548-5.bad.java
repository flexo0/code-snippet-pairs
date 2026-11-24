class Test {
  public String foo(Suit s) {
    switch (s) {
      case SPADE, CLUB -> {
        {
          return "black";
        }
      }
      default -> {
        {
          return "should suggest to remove";
        }
      }
    }
  }
}
