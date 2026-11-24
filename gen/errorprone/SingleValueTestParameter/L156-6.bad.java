public class TestType {
  public void booleans(
      @TestParameter("true") boolean a,
      @TestParameter("True") boolean b,
      @TestParameter("TRUE") Boolean c,
      @TestParameter("false") boolean d,
      @TestParameter("False") boolean e,
      @TestParameter("FALSE") Boolean f) {
    System.out.println(a + " " + b + " " + c + " " + d + " " + e + " " + f);
  }
}
