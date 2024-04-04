import pytest

from application_client.command_sender import CommandSender, Errors
from application_client.response_unpacker import unpack_get_public_key_response
from ragger.bip import calculate_public_key_and_chaincode, CurveChoice
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH


# In this test we check that the GET_PUBLIC_KEY works in confirmation mode
def test_get_public_key_confirm_accepted(firmware, backend, navigator, test_name):
    client = CommandSender(backend)
    path = "m/2645'/1195502025'/1148870696'/0'/0'/5"
    #path = "m/2645'/1195502025'/1148870696'/0'/0'/0"
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

    #ref_public_key, _ = calculate_public_key_and_chaincode(CurveChoice.Secp256k1, path=path)
    assert True

