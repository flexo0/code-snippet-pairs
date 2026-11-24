class Test {
  public String foo(Suit s) {
    return switch (s) {
      case SPADE, CLUB -> "black";
    };
  }
}
