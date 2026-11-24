class Test {
  boolean m(boolean f, Case c) {
    if (f) {
      switch (c) {
        case ONE:
        case TWO:
        case THREE:
          return true;
        default:
          return false;
      }
    }
  }
}
