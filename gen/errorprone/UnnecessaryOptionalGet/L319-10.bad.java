public class Test {
  private void home() {
    opDouble.ifPresent(x -> System.out.println(opDouble.getAsDouble()));
    opInt.ifPresent(x -> System.out.println(opInt.getAsInt()));
    opLong.ifPresent(x -> System.out.println(opLong.getAsLong()));
  }
}
