import pytest

from boilerplate_client.exception import *


@pytest.mark.xfail(raises=ClaNotSupportedError)
def test_bad_cla(cmd):
    sw, _ = cmd.transport.exchange(cla=0xa0,  # 0xa0 instead of 0x5a
                                   ins=0x01,
                                   p1=0x00,
                                   p2=0x00,
                                   cdata=b"")

    raise DeviceException(error_code=sw)


@pytest.mark.xfail(raises=InsNotSupportedError)
def test_bad_ins(cmd):
    sw, _ = cmd.transport.exchange(cla=0x80,
                                   ins=0xfe,  # bad INS
                                   p1=0x00,
                                   p2=0x00,
                                   cdata=b"")

    raise DeviceException(error_code=sw)


#@pytest.mark.xfail(raises=WrongP1P2Error)
#def test_wrong_p1p2(cmd):
#    sw, _ = cmd.transport.exchange(cla=0x80,
#                                   ins=0x01,
#                                   p1=0x08,
#                                   p2=0x00,
#                                   cdata=b"")
#
#    raise DeviceException(error_code=sw)


@pytest.mark.xfail(raises=WrongDataLengthError)
def test_wrong_data_length(cmd):
    # APDUs must be at least 5 bytes: CLA, INS, P1, P2, Lc.
    sw, _ = cmd.transport.exchange_raw("8002000010")

    raise DeviceException(error_code=sw)
