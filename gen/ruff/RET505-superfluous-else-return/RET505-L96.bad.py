def bar4(x):
    if x:  # [no-else-return]
        return True
    else:
        try:
            return False
        except ValueError:
            return None
