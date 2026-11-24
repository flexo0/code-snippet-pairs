public class TestType {
  public void floats(
      @TestParameter("0") float zero,
      @TestParameter("1.5") float a,
      @TestParameter("-2.25") Float b,
      @TestParameter("123") Float fromInt) {
    System.out.println(zero + a + b + fromInt);
  }
}
