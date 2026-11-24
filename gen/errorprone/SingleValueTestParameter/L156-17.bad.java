public class TestType {
  public void ints(
      @TestParameter("0") int zero,
      @TestParameter("42") int a,
      @TestParameter("-7") Integer b,
      @TestParameter("2147483647") int max,
      @TestParameter("-2147483648") int min) {
    System.out.println(zero + a + b + max + min);
  }
}
