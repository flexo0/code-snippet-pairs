class TheLambdasHereAreNotMethods:
    @pytest.mark.parametrize(
        "slicer, expected",
        [
            (lambda x: x[-2:], "foo"),
            (lambda x: x[-5:-3], "bar"),
        ],
    )
    def test_inlet_asset_alias_extra_slice(self, slicer, expected):
        assert slice("whatever") == expected
