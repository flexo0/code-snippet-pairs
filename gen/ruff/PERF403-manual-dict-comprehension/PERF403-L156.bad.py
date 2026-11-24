def foo():
    src = (("x", 1),)
    dst = {}

    for k, v in src:
        if True if True else False:
            dst[k] = v

    for k, v in src:
        if lambda: 0:
            dst[k] = v

# https://github.com/astral-sh/ruff/issues/18859
