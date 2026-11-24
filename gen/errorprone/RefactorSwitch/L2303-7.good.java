class Test {
  public Map<Object, Object> foo(Suit suit) {
    if (toString().length() == 2)
      map =
          switch (suit) {
            case HEART, DIAMOND -> new HashMap<>();
            case SPADE -> throw new RuntimeException();
            default -> throw new NullPointerException();
          };
  }
}
