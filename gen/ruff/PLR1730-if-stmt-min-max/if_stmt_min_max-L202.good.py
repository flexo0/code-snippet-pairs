class Foo:
    def foo(self, value) -> None:
        self._min = min(self._min, value)
        self._max = max(self._max, value)
