async with asyncio.timeout(1), A(), B():
    pass

# SIM117 — nested async with under a sync with parent (issue #27800).
