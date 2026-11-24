def foo1(x, y, z):
    for i in x:
        if i > y:  # [no-else-break]
            break
        a = z
