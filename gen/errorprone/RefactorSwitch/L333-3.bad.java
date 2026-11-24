class Test {
  public boolean foo(Suit suit) {
    for (int i = 0; i < 10; i++) {
      switch (suit) {
        case HEART, CLUB, SPADE -> {
          return true;
        }
        case DIAMOND -> {
          for (int q = 0; q < 10; q++) {
            if (i > 99) {
              continue;
            }
          }
          return false;
        }
        default -> {
          return false;
        }
      }
      // Here is not reachable
    }
  }
}
