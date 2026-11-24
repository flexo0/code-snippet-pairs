class Example {
  void foo(TypeMirror type, Types types, TypeMirror other) {
    if (type.toString().equals(other.toString())) {}
  }
}
