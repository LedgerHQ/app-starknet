import pytest

from application_client.response_unpacker import unpack_get_public_key_response, Errors
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID
from utils import ROOT_SCREENSHOT_PATH, read_lines_from_file

# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_public_key_no_confirm(backend):
   
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, public_key_y = unpack_get_public_key_response(response)

    ref_public_key_x = bytes.fromhex("04ac45fea8814cc2c2bbca343f4280b25d2a5f6d65e511dd16977f35c3e64b74")
    ref_public_key_y = bytes.fromhex("023e4ce66d2d3a466f4326a2def52c68eae80588a36b26574b369d6716fc16bd")
    assert public_key_x == ref_public_key_x
    assert public_key_y == ref_public_key_y


# In this test we check that the GET_PUBLIC_KEY works in confirmation mode
def test_get_public_key_confirm_accepted(firmware, backend, navigator, test_name):
    with backend.exchange_async_raw(bytes.fromhex("5a0101001880000a55c741e9c9c47a6028800000008000000000000000")):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                      [NavInsID.BOTH_CLICK],
                                                      "Approve",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
    response = backend.last_async_response.data
    public_key_x, public_key_y = unpack_get_public_key_response(response)


    ref_public_key_x = bytes.fromhex("04ac45fea8814cc2c2bbca343f4280b25d2a5f6d65e511dd16977f35c3e64b74")
    ref_public_key_y = bytes.fromhex("023e4ce66d2d3a466f4326a2def52c68eae80588a36b26574b369d6716fc16bd")
    assert public_key_x == ref_public_key_x
    assert public_key_y == ref_public_key_y



# In this test we check that the GET_PUBLIC_KEY in confirmation mode replies an error if the user refuses
def test_get_public_key_confirm_refused(firmware, backend, navigator, test_name):
    with pytest.raises(ExceptionRAPDU) as e:
        with backend.exchange_async_raw(bytes.fromhex("5a0101001880000a55c741e9c9c47a6028800000008000000000000000")):
            if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                        [NavInsID.BOTH_CLICK],
                                                        "Reject",
                                                        ROOT_SCREENSHOT_PATH,
                                                        test_name)
            else:
                instructions = [
                    NavInsID.USE_CASE_CHOICE_REJECT,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ]
                navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                            test_name,
                                            instructions)
    assert e.value.status == Errors.SW_DENY
    assert len(e.value.data) == 0