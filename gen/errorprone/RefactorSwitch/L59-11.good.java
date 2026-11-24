class Test {
  public String foo(Suit s) {
    return switch (i) {
      case 2 -> {
        System.out.println("hello");
        if (true) {
          yield "bbb";
        }
        for (int j = 0; j < 10; j++) {
          if (i == 0) {
            yield "for";
          }
          for (String setString : set) {
            yield "enhancedfor";
          }
          while (j > 10) {
            yield "while";
          }
          do {
            if (i == 0) {
              yield "dowhile";
            }
          } while (false);
        }
        Supplier<String> supplier =
            () -> {
              return "lam";
            };
        yield "aaa";
      }
    };
  }
}
