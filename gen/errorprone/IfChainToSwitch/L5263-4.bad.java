class Test {
  String foo(Suit s) {
    class Bar {
      {
        if (s == Suit.HEART) {
          throw new AssertionError("heart");
        } else if (s == Suit.SPADE) {
          throw new AssertionError("spade");
        } else if (s == Suit.DIAMOND) {
          throw new AssertionError("diamond");
        } else if (s == Suit.CLUB) {
          throw new AssertionError("club");
        }
        System.out.println("Don't delete me");
        System.out.println("Don't delete me too");
      }
    }
  }
}
