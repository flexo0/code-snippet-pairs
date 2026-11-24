class Test {
  void f(ClassSymbol symbol) {
    ClassSymbol enclosing = symbol.owner.enclClass();
  }
}
