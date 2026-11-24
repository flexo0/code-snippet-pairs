class Test {
  private void updateScore(Suit suit) {
    switch (suit) {
      case HEART, DIAMOND -> {
        score += -1;
      }
      case SPADE -> {
        score += 2;
      }
      case CLUB -> {
        score += 3;
      }
    }
  }
}
