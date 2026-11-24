def capital(country: str) -> str | None:
    capitals = {
        "England": "London",
        "France": "Paris",
        "Poland": "Warsaw",
        "Romania": "Bucharest",
        "Spain": "Madrid",
        "Thailand": "Bangkok",
    }
    return capitals.get(country)
