public class Test {
  private void home() {
    opDouble.ifPresent(x -> System.out.println(x));
    opInt.ifPresent(x -> System.out.println(x));
    opLong.ifPresent(x -> System.out.println(x));
  }
}
