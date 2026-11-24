def func(a: dict[str, int]) -> list[dict[str, int]]:
    if "services" in a:
        services = a["services"]
        return services


# See: https://github.com/astral-sh/ruff/issues/14052
