class Test {
  void test() {
    List<Integer> bar = new ArrayList<>();
    bar.add(sideEffects());
    List<Integer> baz;
    baz = new ArrayList<>();
    baz.add(sideEffects());
  }
}
