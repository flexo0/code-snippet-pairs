class TheLambdasHereAreNotMethods:
    @pytest.mark.parametrize(
        "slicer, expected",
        [
            (operator.itemgetter(slice(-2, None)), "foo"),
            (operator.itemgetter(slice(-5, -3)), "bar"),
        ],
    )
    def test_inlet_asset_alias_extra_slice(self, slicer, expected):
        assert slice("whatever") == expected
