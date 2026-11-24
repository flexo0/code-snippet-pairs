class Test {
  int f(int x, Object o) {
    switch (x) {
      case 1:
        if (o instanceof String s) {
          return 1;
        } else if (o instanceof Integer i) {
          return 2;
        } else if (o instanceof Object obj) {
          return 3;
        }
        int y = 5;
        return y;
    }
  }
}
