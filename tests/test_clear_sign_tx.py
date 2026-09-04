import pytest

from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_tx_response, Errors
from ragger.backend import RaisePolicy
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH, read_lines_from_file, call_external_binary

CHECK_SIGNATURE_BINARY_PATH = "tools/check-signature/target/debug/check-signature"

# In those tests we check the behavior of the device when asked to sign a Tx (clear signing)

# In this test we send to the device a tx to sign and validate it on screen
# We will ensure that the displayed information is correct by using screenshots comparison
def test_tx_v1_transfer_ETH_0(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v1_transfer_ETH_0.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)), 
                                              [
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.BOTH_CLICK
                                              ],
                                              "Review transaction",
                                              path=ROOT_SCREENSHOT_PATH,
                                              test_case_name=test_name)
        else:
            navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)),
                                                      [
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                          NavInsID.USE_CASE_STATUS_DISMISS
                                                      ], 
                                                      "Review transaction",
                                                      path=ROOT_SCREENSHOT_PATH,
                                                      test_case_name=test_name)
            
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

def test_tx_v1_transfer_ETH_1(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v1_transfer_ETH_1.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)), 
                                              [
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.BOTH_CLICK
                                              ],
                                              "Review transaction",
                                              path=ROOT_SCREENSHOT_PATH,
                                              test_case_name=test_name)
        else:
            navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)),
                                                      [
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                          NavInsID.USE_CASE_STATUS_DISMISS
                                                      ], 
                                                      "Review transaction",
                                                      path=ROOT_SCREENSHOT_PATH,
                                                      test_case_name=test_name)
            
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

def test_tx_v1_transfer_STRK_0(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v1_transfer_STRK_0.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)), 
                                              [
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.BOTH_CLICK
                                              ],
                                              "Review transaction",
                                              path=ROOT_SCREENSHOT_PATH,
                                              test_case_name=test_name)
        else:
            navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)),
                                                      [
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                          NavInsID.USE_CASE_STATUS_DISMISS
                                                      ], 
                                                      "Review transaction",
                                                      path=ROOT_SCREENSHOT_PATH,
                                                      test_case_name=test_name)
            
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


def test_tx_v3_transfer_ETH(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v3_transfer_ETH.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

   # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)), 
                                              [
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.BOTH_CLICK
                                              ],
                                              "Review transaction",
                                              path=ROOT_SCREENSHOT_PATH,
                                              test_case_name=test_name)
        else:
            navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)),
                                                      [
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                          NavInsID.USE_CASE_STATUS_DISMISS
                                                      ], 
                                                      "Review transaction",
                                                      path=ROOT_SCREENSHOT_PATH,
                                                      test_case_name=test_name)
            
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

def test_tx_v3_l1_data_gas_transfer(firmware, backend, navigator, test_name):
        
    # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_v3_l1_data_gas_transfer.dat'
    all_apdus = read_lines_from_file(file_path)
    
    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

   # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)), 
                                              [
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.RIGHT_CLICK,
                                                  NavInsID.BOTH_CLICK
                                              ],
                                              "Review transaction",
                                              path=ROOT_SCREENSHOT_PATH,
                                              test_case_name=test_name)
        else:
            navigator.navigate_until_text_and_compare(NavIns(NavInsID.WAIT, (0,)),
                                                      [
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.SWIPE_CENTER_TO_LEFT,
                                                          NavInsID.USE_CASE_REVIEW_CONFIRM,
                                                          NavInsID.USE_CASE_STATUS_DISMISS
                                                      ], 
                                                      "Review transaction",
                                                      path=ROOT_SCREENSHOT_PATH,
                                                      test_case_name=test_name)
            
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

# A Starknet ERC20 transfer(recipient, amount: u256) is exactly 3 calldata felts:
# recipient, amount low word, amount high word. The clear-sign review only shows
# calldata[0] and calldata[1], so any other calldata shape would sign fields the
# screen never displayed.
#
# The two failure modes are handled differently. A wrong calldata length cannot
# deserialize into the transfer entrypoint on chain, and the app knows that, so
# it refuses the transaction outright. A non-zero high word is a well formed
# transfer the UI simply cannot render, so it falls back to blind signing.
def send_tx_apdus(backend, sample_name):
    # Get the public key first, as the regular clear-sign tests do
    apdus = read_lines_from_file('samples/apdu/dpath_0.dat')
    backend.exchange_raw(bytes.fromhex(apdus[0]))

    all_apdus = read_lines_from_file(f'samples/apdu/{sample_name}.dat')

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    return all_apdus[-1]


def assert_refused(backend, sample_name):
    # Nothing to confirm and no blind-signing prompt offered: the device shows a
    # rejection status on its own and returns the dedicated status word.
    last_apdu = send_tx_apdus(backend, sample_name)

    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    with backend.exchange_async_raw(bytes.fromhex(last_apdu)):
        backend.wait_for_screen_change()

    assert backend.last_async_response.status == Errors.SW_MALFORMED_TRANSFER_CALLDATA


def assert_not_clear_signed(firmware, backend, navigator, sample_name):
    # Blind signing is left disabled here (the default), so the app is expected
    # to offer the "cannot be clear-signed" choice and reject. Navigating until
    # the button text appears keeps this independent of how many pages the
    # prompt spans.
    last_apdu = send_tx_apdus(backend, sample_name)

    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    with backend.exchange_async_raw(bytes.fromhex(last_apdu)):
        if firmware.device.startswith("nano"):
            navigator.navigate_until_text(NavInsID.RIGHT_CLICK,
                                          [NavInsID.BOTH_CLICK],
                                          "Reject transaction")
        else:
            navigator.navigate_until_text(NavIns(NavInsID.WAIT, (0,)),
                                          [NavInsID.USE_CASE_CHOICE_REJECT],
                                          "Reject transaction")

    assert backend.last_async_response.status == Errors.SW_DENY


# A non-zero u256 high word would be signed but never displayed: the review
# would show the low word only, understating the amount by at least 2^128. The
# call is still a valid transfer, so blind signing remains the fallback.
def test_tx_v3_transfer_ETH_high_word_not_clear_signed(firmware, backend, navigator):
    assert_not_clear_signed(firmware, backend, navigator,
                            'tx_v3_transfer_ETH_high_word')


# Extra calldata beyond the 3 expected felts is hashed and signed but never
# shown, and cannot deserialize into transfer() on chain.
def test_tx_v3_transfer_ETH_extra_calldata_refused(backend):
    assert_refused(backend, 'tx_v3_transfer_ETH_extra_calldata')


# Truncated calldata must be caught before the display indexes calldata[1],
# which would otherwise panic the app.
def test_tx_v3_transfer_ETH_short_calldata_refused(backend):
    assert_refused(backend, 'tx_v3_transfer_ETH_short_calldata')
