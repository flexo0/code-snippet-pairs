# Retry.apply

`Retry.apply(input)` calls the function `func` on `input`, again each time it fails, until `exit` says to stop. It
returns the first result; else it throws the last error.

It counts the attempts from 0. Before each one it asks `exit` with the count, and stops when `exit` says yes. The
error to throw starts as an `IllegalArgumentException`, "An immediate exit, didn't have a chance to try at least
once", for when `exit` stops it before the first attempt.

Each attempt:

- calls `func` with `input`, and returns its result if it gives one;
- if `func` is interrupted (`InterruptedException`), sets the thread's interrupted flag again, keeps that as the
  error and stops;
- if it throws any other exception, keeps that as the error;
- then, if the `wait` duration is above zero, sleeps for it; a sleep interrupted is kept as the error and stops it,
  the flag not set again;
- counts one attempt more.

When it stops, it throws the error it kept.
