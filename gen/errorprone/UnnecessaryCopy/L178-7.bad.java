class Test {
  void f(TestProtoMessage m) {
    for (var x : ImmutableList.copyOf(m.getMultiFieldList())) {}
  }
}
