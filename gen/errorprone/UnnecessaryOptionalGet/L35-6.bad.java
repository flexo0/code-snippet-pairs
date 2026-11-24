public class Test {
  private void home() {
    op.ifPresent(x -> System.out.println(op.get()));
    op.map(x -> Long.parseLong(op.get()));
    op.filter(x -> op.get().isEmpty());
    op.flatMap(x -> Optional.of(op.get()));
    op.flatMap(x -> Optional.of(op.orElseThrow()));
  }
}
