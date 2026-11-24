class Test {
  int f(ImmutableList<Object> xs) {
    if (xs.get(0) instanceof Integer i) {
      return i;
    }
  }
}
