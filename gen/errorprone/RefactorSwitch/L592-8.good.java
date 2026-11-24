class Test {
  private String renderName(SideOfCoin sideOfCoin) {
    return switch (sideOfCoin) {
      case OBVERSE -> "Heads";
      case REVERSE -> "Tails";
    };
  }
}
