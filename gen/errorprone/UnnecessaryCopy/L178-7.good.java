class Test {
  void f(TestProtoMessage m) {
    for (var x : m.getMultiFieldList()) {}
  }
}
