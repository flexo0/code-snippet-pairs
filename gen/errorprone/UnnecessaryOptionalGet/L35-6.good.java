public class Test {
  private void home() {
    op.ifPresent(x -> System.out.println(x));
    op.map(x -> Long.parseLong(x));
    op.filter(x -> x.isEmpty());
    op.flatMap(x -> Optional.of(x));
    op.flatMap(x -> Optional.of(x));
  }
}
