import pytest

import json
from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_tx_response, unpack_sign_hash_response, Errors
from ragger.navigator import NavInsID
from utils import ROOT_SCREENSHOT_PATH

from starknet_py.hash.utils import verify_message_signature

# In those tests we check the behavior of the device when asked to sign a Tx (clear or blind signing)

def read_lines_from_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    return [line.strip('=> ').rstrip() for line in lines]

# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_63(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction
    rapdu = backend.exchange_raw(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done

    file_path = 'tools/apdu-generator/apdu_samples/hash_pedersen_63.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
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
                NavInsID.CENTERED_FOOTER_TAP,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.USE_CASE_REVIEW_CONFIRM,
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
            
    response = backend.last_async_response.data
    
    r, s, _ = unpack_sign_hash_response(response)

    # Read hash from a JSON file
    with open('tools/apdu-generator/tx_samples/hash_pedersen_63.json') as f:
        data = json.load(f)
        hash = data['hash']

    assert(
            verify_message_signature(
                msg_hash=int(hash, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )

# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_62(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction
    rapdu = backend.exchange_raw(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done

    file_path = 'tools/apdu-generator/apdu_samples/hash_pedersen_62.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
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
                NavInsID.CENTERED_FOOTER_TAP,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.USE_CASE_REVIEW_CONFIRM,
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
            
    response = backend.last_async_response.data
    
    r, s, _ = unpack_sign_hash_response(response)

    # Read hash from a JSON file
    with open('tools/apdu-generator/tx_samples/hash_pedersen_62.json') as f:
        data = json.load(f)
        hash = data['hash']

    assert(
            verify_message_signature(
                msg_hash=int(hash, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )

# In this test we send to the device a hash to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_sign_hash_61(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction
    rapdu = backend.exchange_raw(bytes.fromhex("5a0100001880000a55c741e9c9c47a6028800000008000000000000000"))
    pub_key_x, _  = unpack_get_public_key_response(rapdu.data)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done

    file_path = 'tools/apdu-generator/apdu_samples/hash_pedersen_61.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
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
                NavInsID.CENTERED_FOOTER_TAP,
                NavInsID.USE_CASE_CHOICE_CONFIRM,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.SWIPE_CENTER_TO_LEFT,
                NavInsID.USE_CASE_REVIEW_CONFIRM,
            ]
            navigator.navigate_and_compare(ROOT_SCREENSHOT_PATH,
                                           test_name,
                                           instructions)
            
    response = backend.last_async_response.data
    
    r, s, _ = unpack_sign_hash_response(response)

    # Read hash from a JSON file
    with open('tools/apdu-generator/tx_samples/hash_pedersen_61.json') as f:
        data = json.load(f)
        hash = data['hash']

    assert(
            verify_message_signature(
                msg_hash=int(hash, 16), 
                signature = [r, s], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )
