import pytest

from application_client.command_sender import CommandSender, Errors
from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_hash_response
from ragger.error import ExceptionRAPDU
from ragger.navigator import NavInsID
from utils import ROOT_SCREENSHOT_PATH

from starknet_py.hash.utils import verify_message_signature

# In this tests we check the behavior of the device when asked to sign a Tx hash

# pedersen hashes
hash_0: str = "55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef" # 31,5 bytes (63 digits)
hash_1: str = "2bd1d3f8f45a011cbd0674ded291d58985761bbcbc04f4d01c8285d1b35c411" # 31,5 bytes (63 digits)
hash_2: str = "2e672d748fbe3b6e833b61ea8b6e688850247022f06406a1eb83e345ffb417"  # 31 bytes (62 digits)
hash_3: str = "936e8798681b391af0c57fe0bf5703b9631bea18b4bc84b3940ebab234744"   # 30,5 bytes (61 digits) 
hash_4: str = "2534b0f53ccac2347dd51befce72338d9b7c568905f17218d93980ce1fc869f" # 31,5 bytes (63 digits)

def fix_sign(hash: str) -> str:
    # fix hash to fit into 32 bytes
    while (len(hash) < 63):
        hash = '0' + hash

    assert(len(hash) == 63)
    return hash + '0'

# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_0(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_0))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_0, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
    
# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_1(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_1))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_1, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
    
# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_2(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_2))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_2, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
    
    # In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_3(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_3))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_3, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
    
# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_3(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_3))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_3, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
    
    # In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_4(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # The path used for this entire test
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    # First we need to get the public key of the device in order to build the transaction
    rapdu = client.get_public_key(path=path)
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_4))):
        # Validate the on-screen request by performing the navigation appropriate for this device
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Approve",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.USE_CASE_REVIEW_TAP,
                                                    [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                    NavInsID.USE_CASE_STATUS_DISMISS],
                                                    "Hold to sign",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)

    # The device as yielded the result, parse it and ensure that the signature is correct
    response = client.get_async_response().data
    r, s, _ = unpack_sign_hash_response(response)

    assert(
            verify_message_signature(
                msg_hash=int(hash_4, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )

# Hash signature refused test
# The test will ask for a transaction hash that will be refused on screen
def test_sign_hash_refused(firmware, backend, navigator, test_name):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    path: str = "m/2645'/1195502025'/1148870696'/0'/0'/5"

    rapdu = client.get_public_key(path=path)
    pub_key_x, _ = unpack_get_public_key_response(rapdu.data)

    if firmware.device.startswith("nano"):
        with pytest.raises(ExceptionRAPDU) as e:
            with client.sign_hash(path=path, hash=bytes.fromhex(fix_sign(hash_0))):
                navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                          [NavInsID.BOTH_CLICK],
                                                          "Reject",
                                                          ROOT_SCREENSHOT_PATH,
                                                          test_name)

        # Assert that we have received a refusal
        assert e.value.status == Errors.SW_DENY
        assert len(e.value.data) == 0
    else:
        for i in range(3):
            instructions = [NavInsID.USE_CASE_REVIEW_TAP] * i
            instructions += [NavInsID.USE_CASE_REVIEW_REJECT,
                             NavInsID.USE_CASE_CHOICE_CONFIRM,
                             NavInsID.USE_CASE_STATUS_DISMISS]
            with pytest.raises(ExceptionRAPDU) as e:
                with client.sign_hash(path=path, hash=hash):
                    navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                                   test_name + f"/part{i}",
                                                   instructions)
            # Assert that we have received a refusal
            assert e.value.status == Errors.SW_DENY
            assert len(e.value.data) == 0
