def foo4(x, y):
    if x:  # [no-else-return]
        if y:
            a = 4
        else:
            b = 2
        return
    c = 3
    return
