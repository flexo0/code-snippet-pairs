class Retry<X, Y> {
    public Y apply(final X input) throws Exception {
        int attempt = 0;
        Exception error = new IllegalArgumentException(
            "An immediate exit, didn't have a chance to try at least once"
        );
        while (!this.exit.apply(attempt)) {
            try {
                return this.func.apply(input);
            } catch (final InterruptedException ex) {
                Thread.currentThread().interrupt();
                error = ex;
                break;
            } catch (final Exception ex) {
                error = ex;
            }
            if (!this.wait.isZero() && !this.wait.isNegative()) {
                try {
                    Thread.sleep(this.wait.toMillis());
                } catch (final InterruptedException ex) {
                    error = ex;
                    break;
                }
            }
            ++attempt;
        }
        throw error;
    }
}
