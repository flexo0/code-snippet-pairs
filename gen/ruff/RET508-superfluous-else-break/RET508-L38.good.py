def foo3(x, y, z):
    for i in x:
        if i > y:
            a = 1
            if z:  # [no-else-break]
                b = 2
                break
            c = 3
            break
        d = 4
        break
