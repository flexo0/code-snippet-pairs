public class TestType {
  public void nullReferenceTypes(
      @TestParameter("null") String s,
      @TestParameter("null") Integer i,
      @TestParameter("null") Long l,
      @TestParameter("null") Float f,
      @TestParameter("null") Double d,
      @TestParameter("null") Boolean b,
      @TestParameter("null") Color c,
      @TestParameter("null") Object o) {
    System.out.println("" + s + i + l + f + d + b + c + o);
  }
}
