class Test {
  public int foo(Suit suit) {
    return switch (z) {
      case "", "DIAMOND", "SPADE" ->
          // Custom comment
          invoke();
      default -> 2;
    };
  }
}
