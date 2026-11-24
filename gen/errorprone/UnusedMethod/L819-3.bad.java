class T {
  void f() {
    var r =
        new Runnable() {
          public void foo() {}

          @Override
          public void run() {}
        };
  }
}
