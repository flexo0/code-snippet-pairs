class Test {
  public int foo(Suit suit, boolean b, boolean c) {
    outer:
    while (true) {
      {
        if (b) {
          break outer;
        }
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
      System.out.println("unreachable after the fix, too");
      if (c) {
        break outer;
      }
    }
  }
}
