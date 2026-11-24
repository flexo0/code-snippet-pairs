Decimal("nan")         # Decimal("nan")
Decimal(" inf ")          # Decimal(" inf ")
Decimal(" +inf ")         # Decimal(" +inf ")
Decimal(" -inf ")         # Decimal(" -inf ")
Decimal(" infinity ")     # Decimal(" infinity ")
Decimal(" +infinity ")    # Decimal(" +infinity ")
Decimal(" -infinity ")    # Decimal(" -infinity ")

# Escape sequence handling in "-nan" case
# Here we do not bother respecting the original whitespace
# and other trivia when offering a fix.
# https://github.com/astral-sh/ruff/issues/16771
