class Example {
  void foo(TypeMirror type, Types types, TypeMirror other) {
    if (types.isSameType(type, other)) {}
  }
}
