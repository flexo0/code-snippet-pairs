def not_ok2():
    if True:
        print(1)
    elif True:
        print(2)
    else:
        if True:
            print(3)
        else:
            print(4)
