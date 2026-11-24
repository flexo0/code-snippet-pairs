def foo6(x, y):
    for i in x:
        if i < x:
            if y:  # [no-else-continue]
                a = 4
                continue
            else:
                b = 2
        else:
            c = 3
        continue
