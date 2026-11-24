async def f():
    with A(), B():
        async with C(), D():
            pass
