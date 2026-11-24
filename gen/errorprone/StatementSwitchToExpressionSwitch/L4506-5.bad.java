class Test {
  private void updateScore(Suit suit) {
    switch (suit) {
      case HEART:
      // Fall through
      case DIAMOND:
        score += -1;
        break;
      case SPADE:
        score += 2;
        break;
      case CLUB:
        score += 3;
    }
  }
}
