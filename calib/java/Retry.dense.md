# Retry.apply

`Y apply(X input) throws Exception`:

- `attempt = 0`; `error = new IllegalArgumentException("An immediate exit, didn't have a chance to try at least
  once")`.
- `while (!exit.apply(attempt))`:
  - `return func.apply(input)`;
  - `catch InterruptedException ex` → `Thread.currentThread().interrupt()`, `error = ex`, `break`;
  - `catch Exception ex` → `error = ex`;
  - `!wait.isZero() && !wait.isNegative()` → `Thread.sleep(wait.toMillis())`; its `InterruptedException ex` →
    `error = ex`, `break`;
  - `++attempt`.
- `throw error`.
