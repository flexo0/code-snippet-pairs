class Test {
  public int foo(Suit suit, boolean b) {
    while (true) {
      switch (suit) {
        case HEART, DIAMOND -> {
          return 1;
        }
        case SPADE -> {
          return 2;
        }
        case CLUB -> {
          return 3;
        }
      }
      System.out.println("unreachable after the fix");
    }
  }
}
