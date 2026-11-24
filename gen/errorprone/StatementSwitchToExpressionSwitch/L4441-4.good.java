class Test {
  public int foo(Suit suit) {
    x *=
        switch (suit) {
          case HEART, DIAMOND, SPADE ->
              /* red suit */
              // Heart comment
              /* red suit */
              // sparkles
              // Diamond comment
              /* black suit */
              2;
          // Before break comment
          // After break comment
          case CLUB ->
              /* black suit */
              // Club comment
              throw new NullPointerException();
            // Club after throw comment
        };
  }
}
