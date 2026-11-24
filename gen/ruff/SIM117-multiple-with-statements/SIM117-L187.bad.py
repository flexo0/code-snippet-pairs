async def f():
    with A():
        with B():
            async with C():
                async with D():
                    pass
