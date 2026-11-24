Decimal(float("\x2dnan"))
Decimal(float("\x20\x2dnan"))
Decimal(float("\x20\u002dnan"))
Decimal(float("\x20\U0000002dnan"))
Decimal(float("\N{space}\N{hyPHen-MINus}nan"))
Decimal(float("\x20\N{character tabulation}\N{hyphen-minus}nan"))
Decimal(float("   -" "nan"))
Decimal(float("-nAn"))

# Test cases for digit separators (safe fixes)
# https://github.com/astral-sh/ruff/issues/20572
