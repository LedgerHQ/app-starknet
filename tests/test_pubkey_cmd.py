import pytest

from application_client.command_sender import CommandSender, Errors
from application_client.response_unpacker import unpack_get_public_key_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in non-confirmation mode
def test_get_public_key_no_confirm(backend):
    for path in ["m/2645'/1195502025'/1148870696'/0'/0'/0"]:
        client = CommandSender(backend)
        response = client.get_public_key(path=path).data
        public_key_x, public_key_y = unpack_get_public_key_response(response)

        #ref_public_key, _ = calculate_public_key_and_chaincode(CurveChoice.Secp256k1, path=path)
        ref_public_key_x = bytes.fromhex("04ac45fea8814cc2c2bbca343f4280b25d2a5f6d65e511dd16977f35c3e64b74")
        ref_public_key_y = bytes.fromhex("023e4ce66d2d3a466f4326a2def52c68eae80588a36b26574b369d6716fc16bd")
        assert public_key_x == ref_public_key_x
        assert public_key_y == ref_public_key_y


# In this test we check that the GET_PUBLIC_KEY works in confirmation mode
def test_get_public_key_confirm_accepted(firmware, backend, navigator, test_name):
    client = CommandSender(backend)
    path = "m/2645'/1195502025'/1148870696'/0'/0'/0"
    with client.get_public_key_with_confirmation(path=path):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                      [NavInsID.BOTH_CLICK],
                                                      "Approve",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
        else:
            instructions = [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavIns(NavInsID.TOUCH, (200, 335)),
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_EXIT_QR,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
    response = client.get_async_response().data
    public_key_x, public_key_y = unpack_get_public_key_response(response)


    ref_public_key_x = bytes.fromhex("04ac45fea8814cc2c2bbca343f4280b25d2a5f6d65e511dd16977f35c3e64b74")
    ref_public_key_y = bytes.fromhex("023e4ce66d2d3a466f4326a2def52c68eae80588a36b26574b369d6716fc16bd")
    assert public_key_x == ref_public_key_x
    assert public_key_y == ref_public_key_y



# In this test we check that the GET_PUBLIC_KEY in confirmation mode replies an error if the user refuses
def test_get_public_key_confirm_refused(firmware, backend, navigator, test_name):
    client = CommandSender(backend)
    path = "m/2645'/1195502025'/1148870696'/0'/0'/0"

    if firmware.device.startswith("nano"):
        with pytest.raises(ExceptionRAPDU) as e:
            with client.get_public_key_with_confirmation(path=path):
                navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                          [NavInsID.BOTH_CLICK],
                                                          "Reject",
                                                          ROOT_SCREENSHOT_PATH,
                                                          test_name)
        # Assert that we have received a refusal
        assert e.value.status == Errors.SW_DENY
        assert len(e.value.data) == 0
    else:
        instructions_set = [
            [
                NavInsID.USE_CASE_REVIEW_REJECT,
                NavInsID.USE_CASE_STATUS_DISMISS
            ],
            [
                NavInsID.USE_CASE_REVIEW_TAP,
                NavInsID.USE_CASE_ADDRESS_CONFIRMATION_CANCEL,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
        ]
        for i, instructions in enumerate(instructions_set):
            with pytest.raises(ExceptionRAPDU) as e:
                with client.get_public_key_with_confirmation(path=path):
                    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                                   test_name + f"/part{i}",
                                                   instructions)
            # Assert that we have received a refusal
            assert e.value.status == Errors.SW_DENY
            assert len(e.value.data) == 0
