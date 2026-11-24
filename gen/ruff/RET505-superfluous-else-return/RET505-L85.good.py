def foo6(x, y):
    if x:
        if y:  # [no-else-return]
            a = 4
            return
        b = 2
    else:
        c = 3
