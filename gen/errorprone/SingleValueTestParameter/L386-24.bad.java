public class TestType {
  public void multipleInlinedWithOneKept(
      @TestParameter("1") int a, @TestParameter boolean kept, @TestParameter("foo") String b) {
    System.out.println(a + " " + kept + " " + b);
  }
}
