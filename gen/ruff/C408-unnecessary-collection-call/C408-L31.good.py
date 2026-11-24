t"{ {'x': 'y'} | {'y': 'z'} }"
t"{ {'x': 'y'} | {'y': 'z'} }"
t"a { {'x': 'y'} | {'y': 'z'} } b"
t"a { {'x': 'y'} | {'y': 'z'} } b"

# https://github.com/astral-sh/ruff/issues/16234
# Python normalizes identifiers to NFKC, but does not normalize string literals, so the fix has to
# normalize the keyword name to preserve the dictionary key at runtime. The character "ℼ" normalizes
# to "π", and "ſ" normalizes to "s".
