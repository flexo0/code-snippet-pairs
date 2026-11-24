public class Test {
  private void home() {
    op.ifPresent(x -> System.out.println(op.orElse("other")));
    op.ifPresent(x -> System.out.println(op.orElseGet(() -> "other")));
    op.ifPresent(x -> System.out.println(op.orElseThrow(RuntimeException::new)));
  }
}
