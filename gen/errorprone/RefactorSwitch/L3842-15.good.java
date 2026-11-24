class Test {
  public String foo(Suit s) {
    return switch (s) {
      case CLUB -> {
        if (true) {
          yield makeString();
        } else {
          System.out.println("Desktop MinuteMaid URLs not supported for flow: " + s);
          yield makeString();
        }
      }
    };
  }
}
