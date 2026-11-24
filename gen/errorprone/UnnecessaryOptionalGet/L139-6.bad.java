public class Test {
  private void home() {
    op.transform(x -> Long.parseLong(op.or("other")));
    op.transform(x -> Long.parseLong(op.or(() -> "other")));
  }
}
