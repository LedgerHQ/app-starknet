import pytest

from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_tx_response, Errors
from ragger.navigator import NavInsID
from utils import ROOT_SCREENSHOT_PATH

from starknet_py.hash.utils import verify_message_signature

# In those tests we check the behavior of the device when asked to sign a Tx (clear or blind signing)

# In this test we send to the device a tx to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_clear_sign_tx(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction
    rapdu = backend.exchange_raw(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    apdus = [
        bytes.fromhex("5a0300001880000a55c741e9c9c47a6028800000008000000000000000"),
        bytes.fromhex("5a030100e007e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a000000000000000000000000000000000000000000000000000000000000000000004c315f47415300000000000000000000000000000000000000000000000000004c325f47415300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000534e5f4d41494e00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000"),
        bytes.fromhex("5a03020000"),
        bytes.fromhex("5a03030000"),
        bytes.fromhex("5a030400200000000000000000000000000000000000000000000000000000000000000001"),
        bytes.fromhex("5a03050080049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc70083afd3f4caedc6eebf44246fe54e38c95e3179a5ec9ea81740eca5b482d12e07e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a00000000000000000000000000000000000000000000000000000000000003e8"),
        bytes.fromhex("5a03050200"),
    ]
    # send all apdus except last one
    for apdu in apdus[:-1]:
        backend.exchange_raw(apdu)
    # send last apdu and yield the response
    with backend.exchange_async_raw(apdus[-1]):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            instructions = [
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.USE_CASE_REVIEW_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
            
    response = backend.last_async_response.data
    
    hash, r, s, _ = unpack_sign_tx_response(response)

    print("hash: ", hash)

    assert(
            verify_message_signature(
                msg_hash=hash, 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )

def test_blind_sign_tx(firmware, backend, navigator, test_name):
    
    # Use the app interface instead of raw interface
    #client = CommandSender(backend)
    
    # First we need to get the public key of the device in order to build the transaction
    #rapdu = client.get_public_key(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    rapdu = backend.exchange_raw(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    apdus = [
        bytes.fromhex("5a0300001880000a55c741e9c9c47a6028800000008000000000000000"),
        bytes.fromhex("5a030100e007e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a000000000000000000000000000000000000000000000000000000000000000000004c315f47415300000000000000000000000000000000000000000000000000004c325f47415300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000534e5f4d41494e00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000"),
        bytes.fromhex("5a03020000"),
        bytes.fromhex("5a03030000"),
        bytes.fromhex("5a030400200000000000000000000000000000000000000000000000000000000000000001"),
        bytes.fromhex("5a03050080049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc702f0b3c5710379609eb5495f1ecd348cb28167711b73609fe565a7273455035407e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a00000000000000000000000000000000000000000000000000000000000003e8"),
        bytes.fromhex("5a03050200"),
    ]
    # send all apdus except last one
    for apdu in apdus[:-1]:
        backend.exchange_raw(apdu)
    # send last apdu and yield the response
    with backend.exchange_async_raw(apdus[-1]):
        if firmware.device.startswith("nano"):
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                             test_name,
                                             [NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.BOTH_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.RIGHT_CLICK,
                                              NavInsID.BOTH_CLICK])
        else:
            instructions = [
                NavInsID.USE_CASE_STATUS_DISMISS,
                NavInsID.CENTERED_FOOTER_TAP,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.USE_CASE_REVIEW_CONFIRM,
                NavInsID.USE_CASE_STATUS_DISMISS
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
    
    response = backend.last_async_response.data
    
    hash, r, s, _ = unpack_sign_tx_response(response)

    assert(
            verify_message_signature(
                msg_hash=hash, 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )