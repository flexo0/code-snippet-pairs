class Test {
  int f(ImmutableList<Object> xs) {
    if (xs.get(0) instanceof Integer) {
      return (Integer) xs.get(0);
    }
  }
}
