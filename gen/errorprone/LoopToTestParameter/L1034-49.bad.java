public class TestType {
  public void existingParam(@TestParameter int a) {
    for (int b : List.of(1, 2)) {
      System.out.println(a + b);
    }
  }
}
