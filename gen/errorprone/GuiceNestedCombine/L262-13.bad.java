class Test {
  public void test() {
    foo(
        new ModuleA(),
        Modules.combine(new ModuleB(), new ModuleC()),
        Modules.combine(new ModuleB(), new ModuleC()));
  }
}
