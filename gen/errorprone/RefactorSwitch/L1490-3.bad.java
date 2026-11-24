class Test {
  public int foo(Suit suit, boolean c) {
    outer:
    while (true) {
      {
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
        if (c) {
          break outer;
        }
      }
      System.out.println("unreachable after the fix");
    }
    return 0;
  }
}
