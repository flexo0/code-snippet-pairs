class Test {
  int f(int x, Object o) {
    switch (x) {
      case 1:
        switch (o) {
          case String s -> {
            return 1;
          }
          case Integer i -> {
            return 2;
          }
          case Object obj -> {
            return 3;
          }
        }
    }
  }
}
