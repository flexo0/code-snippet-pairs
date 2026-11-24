def foo():
    src = (("x", 1),)

    dst = {k: v for k, v in src if (True if True else False)}

    dst.update({k: v for k, v in src if (lambda: 0)})

# https://github.com/astral-sh/ruff/issues/18859
