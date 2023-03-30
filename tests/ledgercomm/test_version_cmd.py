def test_version(cmd):
    assert cmd.get_version() == (0, 1, 0)
