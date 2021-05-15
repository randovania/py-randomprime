import unittest.mock

import py_randomprime


def test_patch_iso(temp_path):
    m = unittest.mock.patch("py_randomprime.rust.patch_iso")

    py_randomprime.patch_iso(
        temp_path.joinpath("input.iso"),
        temp_path.joinpath("output.iso"),
        {},
        py_randomprime.ProgressNotifier(print),
    )

    m.assert_called_once()
