class Test {
  public int foo(Suit suit, boolean b, boolean c) {
    outer:
    do {
      {
        if (b) {
          continue outer;
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
    } while (c);
  }
}
