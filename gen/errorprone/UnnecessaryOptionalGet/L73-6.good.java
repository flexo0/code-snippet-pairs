public class Test {
  private void home() {
    op.transform(x -> Long.parseLong(x));
    op.transform((String x) -> Long.parseLong(x));
  }
}
