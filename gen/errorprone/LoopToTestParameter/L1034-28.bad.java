public class TestType {
  public void immutableSetOfTest() {
    for (double d : ImmutableSet.of(1.5, -2.5)) {
      System.out.println(d);
    }
  }
}
