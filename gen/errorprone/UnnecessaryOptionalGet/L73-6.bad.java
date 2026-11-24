public class Test {
  private void home() {
    op.transform(x -> Long.parseLong(op.get()));
    op.transform((String x) -> Long.parseLong(op.get()));
  }
}
