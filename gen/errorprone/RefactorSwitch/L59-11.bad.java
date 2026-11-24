class Test {
  public String foo(Suit s) {
    switch (i) {
      case 2 -> {
        System.out.println("hello");
        if (true) {
          return "bbb";
        }
        for (int j = 0; j < 10; j++) {
          if (i == 0) {
            return "for";
          }
          for (String setString : set) {
            return "enhancedfor";
          }
          while (j > 10) {
            return "while";
          }
          do {
            if (i == 0) {
              return "dowhile";
            }
          } while (false);
        }
        Supplier<String> supplier =
            () -> {
              return "lam";
            };
        return "aaa";
      }
    }
  }
}
