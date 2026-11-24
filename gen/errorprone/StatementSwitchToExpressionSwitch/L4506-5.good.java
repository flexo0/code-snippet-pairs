class Test {
  private void updateScore(Suit suit) {
    score +=
        switch (suit) {
          case HEART, DIAMOND -> -1;
          case SPADE -> 2;
          case CLUB -> 3;
        };
  }
}
