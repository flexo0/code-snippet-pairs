t"{dict(x='y')}"
t'{dict(x="y")}'
t"{dict()}"
t"a {dict()} b"

t"{dict(x='y') | dict(y='z')}"
t"{ dict(x='y') | dict(y='z') }"
t"a {dict(x='y') | dict(y='z')} b"
t"a { dict(x='y') | dict(y='z') } b"

# https://github.com/astral-sh/ruff/issues/16234
# Python normalizes identifiers to NFKC, but does not normalize string literals, so the fix has to
# normalize the keyword name to preserve the dictionary key at runtime. The character "ℼ" normalizes
# to "π", and "ſ" normalizes to "s".
