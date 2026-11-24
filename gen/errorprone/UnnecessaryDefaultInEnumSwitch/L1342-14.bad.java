class Test {
  public static void main(String[] args) {
    switch (type) {
      case BAR, BAZ -> {}
      default -> throw new AssertionError(type);
    }
  }
}
