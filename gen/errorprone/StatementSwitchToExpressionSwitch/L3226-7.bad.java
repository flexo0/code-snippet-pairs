class Test {
  public Map<Object, Object> foo(Suit suit) {
    if (toString().length() == 2)
      switch (suit) {
        case HEART:
        case DIAMOND:
          map = new HashMap<>();
          break;
        case SPADE:
          throw new RuntimeException();
        default:
          throw new NullPointerException();
      }
  }
}
