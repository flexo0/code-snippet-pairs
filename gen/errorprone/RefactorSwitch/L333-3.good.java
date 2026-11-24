class Test {
  public boolean foo(Suit suit) {
    for (int i = 0; i < 10; i++) {
      return switch (suit) {
        case HEART, CLUB, SPADE -> true;
        case DIAMOND -> {
          for (int q = 0; q < 10; q++) {
            if (i > 99) {
              continue;
            }
          }
          yield false;
        }
        default -> false;
      };
      // Here is not reachable
    }
  }
}
