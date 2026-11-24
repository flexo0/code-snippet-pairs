def bar4(x):
    if x:  # [no-else-return]
        return True
    try:
        return False
    except ValueError:
        return None
