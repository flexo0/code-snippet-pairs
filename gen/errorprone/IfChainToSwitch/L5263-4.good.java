class Test {
  String foo(Suit s) {
    class Bar {
      {
        switch (s) {
          case Suit.HEART -> throw new AssertionError("heart");
          case Suit.SPADE -> throw new AssertionError("spade");
          case Suit.DIAMOND -> throw new AssertionError("diamond");
          case Suit.CLUB -> throw new AssertionError("club");
        }
        System.out.println("Don't delete me");
        System.out.println("Don't delete me too");
      }
    }
  }
}
