class Test {
  void test() {
    List<Integer> foo = new ArrayList<>();
    foo.add(1);
    List<Integer> bar = new ArrayList<>();
    bar.add(sideEffects());
    List<Integer> baz;
    baz = new ArrayList<>();
    baz.add(sideEffects());
  }
}
