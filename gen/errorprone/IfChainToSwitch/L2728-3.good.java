class Test {
  public void foo(Suit s) {
    {
      Suit suit = s;
      switch (suit) {
        case Suit.SPADE -> {
          return;
        }
        case Suit.DIAMOND -> {
          return;
        }
        case Suit.HEART -> {
          return;
        }
        case Suit.CLUB -> {
          return;
        }
        case null -> {
          return;
        }
      }
    }
  }
}
