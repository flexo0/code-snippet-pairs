class Test {
  private String renderName(SideOfCoin sideOfCoin) {
    switch (sideOfCoin) {
      case OBVERSE -> {
        return "Heads";
      }
      case REVERSE -> {
        return "Tails";
      }
    }
    // This should never happen, but removing this will cause a compile-time error
    throw new RuntimeException("Unknown side of coin");
  }
}
