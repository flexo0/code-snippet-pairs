def foo4(x, y):
    if x:  # [no-else-raise]
        if y:
            a = 4
        else:
            b = 2
        raise Exception(x)
    c = 3
    raise Exception(y)
