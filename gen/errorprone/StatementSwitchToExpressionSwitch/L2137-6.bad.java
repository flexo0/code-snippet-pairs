class Test {
  public int foo(Suit suit) {
    if (x < 4) {
      try {
        System.out.println("don't delete 0");
        synchronized (this) {
          try {
            System.out.println("don't delete 1");
            System.out.println("don't delete 11");
            switch (suit) {
              case HEART:
              case DIAMOND:
              case SPADE:
                return 1;
              case CLUB:
                throw new NullPointerException();
            }
            System.out.println("delete me");
          } catch (Throwable e) {
            throw new RuntimeException("rethrew");
          }
        }

        // Becomes unreachable
        System.out.println("uh-oh");
      } catch (Throwable e) {
        throw new RuntimeException("rethrew");
      }
      // Also becomes unreachable
      System.out.println("delete me too");
      System.out.println("delete me three");
      return 3;
    }
  }
}
