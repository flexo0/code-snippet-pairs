class Test {
  public int[] foo(Suit suit) {
    switch (suit) {
      case HEART -> throw new RuntimeException();
      case DIAMOND -> {
        x[6] <<= (((x[6] + 1) * (x[6] * x[5]) << 1));
      }
      case SPADE -> throw new RuntimeException();
      default -> throw new NullPointerException();
    }
  }
}
