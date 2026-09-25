import pytest

from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_tx_response, Errors
from ragger.backend import RaisePolicy
from ragger.navigator import NavInsID
from utils import ROOT_SCREENSHOT_PATH, read_lines_from_file, call_external_binary

CHECK_SIGNATURE_BINARY_PATH = "tools/check-signature/target/debug/check-signature"

# Reference transaction hashes, computed with starknet-core from the JSON samples.
TX_V1_DEPLOY_ACCOUNT_HASH = "074c150ef8bc9f0495bc6265be47706bc21f89105518fe8e93b306d3bcad2304"
TX_V1_DEPLOY_ACCOUNT_8_ARGS_HASH = "028a823f9b5845c67e79a0a1c129d136972d4b08df69b62adc87c751fb052c95"

# In those tests we check the behavior of the device when asked to sign a DEPLOY_ACCOUNT Tx (clear signing)

# In this test we send to the device a tx to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_tx_v3_deploy_account(firmware, backend, navigator, test_name):
        
    # We need to get the public key of the device to check the signature   
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v3_deploy_account.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Sign Transaction ?",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            # The review spans more pages as constructor arguments are added.
            navigator.navigate_until_text_and_compare(NavInsID.SWIPE_CENTER_TO_LEFT,
                                                      [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                       NavInsID.USE_CASE_STATUS_DISMISS],
                                                      "Sign Transaction ?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
            
    response = backend.last_async_response.data
    
    hash, r, s, _ = unpack_sign_tx_response(response)

    # Call the external binary with the signature and the public key
    binary_path = CHECK_SIGNATURE_BINARY_PATH
    args = ["-t", hash.hex(),
            "-p", public_key_x.hex(),
            "-r", r.hex(),
            "-s", s.hex()]
    stdout, stderr = call_external_binary(binary_path, *args)

    if stdout:
        # Convert the output to a boolean value
        result = stdout.lower() == "true"
        print(f"Result as boolean: {result}")
        assert(result)
    if stderr:
        print("Standard Error:")
        print(stderr)
        assert(False)

def test_tx_v1_deploy_account(firmware, backend, navigator, test_name):
        
    # We need to get the public key of the device to check the signature   
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v1_deploy_account.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Sign Transaction ?",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            # The review spans more pages as constructor arguments are added.
            navigator.navigate_until_text_and_compare(NavInsID.SWIPE_CENTER_TO_LEFT,
                                                      [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                       NavInsID.USE_CASE_STATUS_DISMISS],
                                                      "Sign Transaction ?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
            
    response = backend.last_async_response.data
    
    hash, r, s, _ = unpack_sign_tx_response(response)
    # Reference hash computed with starknet-core from samples/tx/tx_v1_deploy_account.json
    assert hash.hex() == TX_V1_DEPLOY_ACCOUNT_HASH

    # Call the external binary with the signature and the public key
    binary_path = CHECK_SIGNATURE_BINARY_PATH
    args = ["-t", hash.hex(),
            "-p", public_key_x.hex(),
            "-r", r.hex(),
            "-s", s.hex()]
    stdout, stderr = call_external_binary(binary_path, *args)

    if stdout:
        # Convert the output to a boolean value
        result = stdout.lower() == "true"
        print(f"Result as boolean: {result}")
        assert(result)
    if stderr:
        print("Standard Error:")
        print(stderr)
        assert(False)

def test_tx_v3_l1_data_gas_deploy_account(firmware, backend, navigator, test_name):
        
    # We need to get the public key of the device to check the signature   
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v3_l1_data_gas_deploy_account.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                    [NavInsID.BOTH_CLICK],
                                                    "Sign Transaction ?",
                                                    ROOT_SCREENSHOT_PATH,
                                                    test_name)
        else:
            # The review spans more pages as constructor arguments are added.
            navigator.navigate_until_text_and_compare(NavInsID.SWIPE_CENTER_TO_LEFT,
                                                      [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                       NavInsID.USE_CASE_STATUS_DISMISS],
                                                      "Sign Transaction ?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
            
    response = backend.last_async_response.data
    
    hash, r, s, _ = unpack_sign_tx_response(response)

    # Call the external binary with the signature and the public key
    binary_path = CHECK_SIGNATURE_BINARY_PATH
    args = ["-t", hash.hex(),
            "-p", public_key_x.hex(),
            "-r", r.hex(),
            "-s", s.hex()]
    stdout, stderr = call_external_binary(binary_path, *args)

    if stdout:
        # Convert the output to a boolean value
        result = stdout.lower() == "true"
        print(f"Result as boolean: {result}")
        assert(result)
    if stderr:
        print("Standard Error:")
        print(stderr)
        assert(False)

# The tip is signed along with the resource bounds of a V3 deployment too: the
# review shows it on its own line and counts it towards Max Fees.
def test_tx_v3_deploy_account_tip(firmware, backend, navigator, test_name):
    apdus = read_lines_from_file('samples/apdu/dpath_0.dat')
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    all_apdus = read_lines_from_file('samples/apdu/tx_v3_deploy_account_tip.dat')
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text_and_compare(NavInsID.RIGHT_CLICK,
                                                      [NavInsID.BOTH_CLICK],
                                                      "Sign Transaction ?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)
        else:
            navigator.navigate_until_text_and_compare(NavInsID.SWIPE_CENTER_TO_LEFT,
                                                      [NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                       NavInsID.USE_CASE_STATUS_DISMISS],
                                                      "Sign Transaction ?",
                                                      ROOT_SCREENSHOT_PATH,
                                                      test_name)

    hash, r, s, _ = unpack_sign_tx_response(backend.last_async_response.data)
    stdout, stderr = call_external_binary(CHECK_SIGNATURE_BINARY_PATH,
                                          "-t", hash.hex(),
                                          "-p", public_key_x.hex(),
                                          "-r", r.hex(),
                                          "-s", s.hex())
    assert not stderr, stderr
    assert stdout.lower() == "true"


# The host supplies the account address, but the account actually deployed is
# the one derived from the signed class hash, salt and constructor calldata.
# When the two disagree the review would show an address that is not the one
# being deployed, so the device refuses before showing anything.
def test_tx_v3_deploy_account_wrong_address_refused(backend):
    apdus = read_lines_from_file('samples/apdu/dpath_0.dat')
    backend.exchange_raw(bytes.fromhex(apdus[0]))

    all_apdus = read_lines_from_file('samples/apdu/tx_v3_deploy_account_wrong_address.dat')
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        backend.wait_for_screen_change()

    assert backend.last_async_response.status == Errors.SW_CONTRACT_ADDRESS_MISMATCH


def send_until_last(backend, sample_name):
    apdus = read_lines_from_file('samples/apdu/dpath_0.dat')
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    all_apdus = read_lines_from_file(f'samples/apdu/{sample_name}.dat')
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))
    return public_key_x, all_apdus[-1]


def assert_signature(public_key_x, response):
    hash, r, s, _ = unpack_sign_tx_response(response)
    stdout, stderr = call_external_binary(CHECK_SIGNATURE_BINARY_PATH,
                                          "-t", hash.hex(),
                                          "-p", public_key_x.hex(),
                                          "-r", r.hex(),
                                          "-s", s.hex())
    assert not stderr, stderr
    assert stdout.lower() == "true"
    return hash


def navigate_to_sign(firmware, navigator, test_name=None):
    if firmware.device.startswith("nano"):
        nav, validation = NavInsID.RIGHT_CLICK, [NavInsID.BOTH_CLICK]
    else:
        nav, validation = (NavInsID.SWIPE_CENTER_TO_LEFT,
                           [NavInsID.USE_CASE_REVIEW_CONFIRM, NavInsID.USE_CASE_STATUS_DISMISS])
    if test_name:
        navigator.navigate_until_text_and_compare(nav, validation, "Sign Transaction ?",
                                                  ROOT_SCREENSHOT_PATH, test_name)
    else:
        navigator.navigate_until_text(nav, validation, "Sign Transaction ?")


# A constructor without arguments is sent as a single empty calldata chunk.
def test_tx_v3_deploy_account_zero_args(firmware, backend, navigator, test_name):
    public_key_x, last_apdu = send_until_last(backend, 'tx_v3_deploy_account_zero_args')
    with backend.exchange_async_raw(bytes.fromhex(last_apdu)):
        navigate_to_sign(firmware, navigator, test_name)
    assert_signature(public_key_x, backend.last_async_response.data)


# Eight constructor arguments span two calldata chunks. Every one of them is
# reviewed, and the signed hash is the one Starknet computes for the whole
# calldata, not one absorbed chunk by chunk.
def test_tx_v1_deploy_account_8_args(firmware, backend, navigator):
    public_key_x, last_apdu = send_until_last(backend, 'tx_v1_deploy_account_8_args')
    with backend.exchange_async_raw(bytes.fromhex(last_apdu)):
        navigate_to_sign(firmware, navigator)
    hash = assert_signature(public_key_x, backend.last_async_response.data)
    assert hash.hex() == TX_V1_DEPLOY_ACCOUNT_8_ARGS_HASH


def test_tx_v1_deploy_account_wrong_address_refused(backend):
    _, last_apdu = send_until_last(backend, 'tx_v1_deploy_account_wrong_address')
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    with backend.exchange_async_raw(bytes.fromhex(last_apdu)):
        backend.wait_for_screen_change()
    assert backend.last_async_response.status == Errors.SW_CONTRACT_ADDRESS_MISMATCH
