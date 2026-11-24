Decimal(float(" -nan "))         # Decimal("nan")
Decimal(float(" inf "))          # Decimal(" inf ")
Decimal(float(" +inf "))         # Decimal(" +inf ")
Decimal(float(" -inf "))         # Decimal(" -inf ")
Decimal(float(" infinity "))     # Decimal(" infinity ")
Decimal(float(" +infinity "))    # Decimal(" +infinity ")
Decimal(float(" -infinity "))    # Decimal(" -infinity ")

# Escape sequence handling in "-nan" case
# Here we do not bother respecting the original whitespace
# and other trivia when offering a fix.
# https://github.com/astral-sh/ruff/issues/16771
