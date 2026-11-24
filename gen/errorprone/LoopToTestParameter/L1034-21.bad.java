public class TestType {
  public void setOfTest() {
    for (int x : Set.of(+1, (-2), -(3))) {
      System.out.println(x);
    }
  }
}
