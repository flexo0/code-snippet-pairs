def handle_surrogates():
    # should be linted
    text = "\ud800\udc00heythere"
    text = text.removeprefix("\ud800\udc00")
    text = "\U00010000heythere"
    text = text.removeprefix("\U00010000")
    
    # should not be linted
    text = "\ud800\udc00heythere"
    if text.startswith("\ud800\udc00"):
        text = text[1:]

# Regression test for
# https://github.com/astral-sh/ruff/issues/16231
