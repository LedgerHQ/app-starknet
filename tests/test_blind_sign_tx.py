import pytest

from application_client.response_unpacker import unpack_get_public_key_response, unpack_sign_tx_response, Errors
from ragger.navigator import NavInsID, NavIns
from utils import ROOT_SCREENSHOT_PATH, read_lines_from_file, call_external_binary
from ragger.firmware import Firmware

CHECK_SIGNATURE_BINARY_PATH = "./target/debug/check-signature"

def get_setting_position(firmware: Firmware, setting_idx: int, per_page: int) -> tuple[int, int]:
    if firmware == Firmware.STAX:
        screen_height = 672  # px
        screen_width = 400  # px
        header_height = 88  # px
        footer_height = 92  # px
    else:
        screen_height = 600  # px
        screen_width = 480  # px
        header_height = 96  # px
        footer_height = 96  # px

    index_in_page = setting_idx % per_page
    usable_height = screen_height - (header_height + footer_height)
    setting_height = usable_height // per_page
    offset = (setting_height * index_in_page) + (setting_height // 2)
    return screen_width // 2, header_height + offset

# In those tests we check the behavior of the device when asked to sign a Tx (blind signing)

def test_blind_sign_tx_0(firmware, backend, navigator, test_name):

    # Enable blind siging in settings
    if firmware.device.startswith("nano"):

        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    else:
        settings_per_page = 3 if firmware == Firmware.STAX else 2
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, get_setting_position(firmware, 0, settings_per_page)),
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    
     # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_misc_0.dat'
    all_apdus = read_lines_from_file(file_path)

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(
                    NavIns(NavInsID.WAIT, (0,)),
                    [
                        NavInsID.BOTH_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.BOTH_CLICK
                    ],
                    "Blind",
                    path=ROOT_SCREENSHOT_PATH,
                    test_case_name=test_name
                )
        else:
            navigator.navigate_until_text_and_compare(
                NavIns(NavInsID.WAIT, (0,)),
                [
                    NavInsID.CENTERED_FOOTER_TAP,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.USE_CASE_REVIEW_CONFIRM,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ],
                "Blind signing ahead",
                path=ROOT_SCREENSHOT_PATH,
                test_case_name=test_name
            )
    
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

def test_blind_sign_tx_1(firmware, backend, navigator, test_name):

     # Enable blind siging in settings
    if firmware.device.startswith("nano"):

        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    else:
        settings_per_page = 3 if firmware == Firmware.STAX else 2
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, get_setting_position(firmware, 0, settings_per_page)),
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    
     # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_misc_1.dat'
    all_apdus = read_lines_from_file(file_path)

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(
                    NavIns(NavInsID.WAIT, (0,)),
                    [
                        NavInsID.BOTH_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.BOTH_CLICK
                    ],
                    "Blind",
                    path=ROOT_SCREENSHOT_PATH,
                    test_case_name=test_name
                )
        else:
            navigator.navigate_until_text_and_compare(
                NavIns(NavInsID.WAIT, (0,)),
                [
                    NavInsID.CENTERED_FOOTER_TAP,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.USE_CASE_REVIEW_CONFIRM,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ],
                "Blind signing ahead",
                path=ROOT_SCREENSHOT_PATH,
                test_case_name=test_name
            )
    
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

def test_blind_sign_tx_2(firmware, backend, navigator, test_name):

     # Enable blind siging in settings
    if firmware.device.startswith("nano"):

        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    else:
        settings_per_page = 3 if firmware == Firmware.STAX else 2
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, get_setting_position(firmware, 0, settings_per_page)),
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    
     # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_misc_2.dat'
    all_apdus = read_lines_from_file(file_path)

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(
                    NavIns(NavInsID.WAIT, (0,)),
                    [
                        NavInsID.BOTH_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.BOTH_CLICK
                    ],
                    "Blind",
                    path=ROOT_SCREENSHOT_PATH,
                    test_case_name=test_name
                )
        else:
            navigator.navigate_until_text_and_compare(
                NavIns(NavInsID.WAIT, (0,)),
                [
                    NavInsID.CENTERED_FOOTER_TAP,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.USE_CASE_REVIEW_CONFIRM,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ],
                "Blind signing ahead",
                path=ROOT_SCREENSHOT_PATH,
                test_case_name=test_name
            )
    
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

def test_blind_sign_tx_3(firmware, backend, navigator, test_name):

     # Enable blind siging in settings
    if firmware.device.startswith("nano"):

        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    else:
        settings_per_page = 3 if firmware == Firmware.STAX else 2
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, get_setting_position(firmware, 0, settings_per_page)),
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    
     # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_misc_3.dat'
    all_apdus = read_lines_from_file(file_path)

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(
                    NavIns(NavInsID.WAIT, (0,)),
                    [
                        NavInsID.BOTH_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.BOTH_CLICK
                    ],
                    "Blind",
                    path=ROOT_SCREENSHOT_PATH,
                    test_case_name=test_name
                )
        else:
            navigator.navigate_until_text_and_compare(
                NavIns(NavInsID.WAIT, (0,)),
                [
                    NavInsID.CENTERED_FOOTER_TAP,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.USE_CASE_REVIEW_CONFIRM,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ],
                "Blind signing ahead",
                path=ROOT_SCREENSHOT_PATH,
                test_case_name=test_name
            )
    
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

def test_blind_sign_tx_4(firmware, backend, navigator, test_name):

     # Enable blind siging in settings
    if firmware.device.startswith("nano"):

        instructions = [
            NavInsID.RIGHT_CLICK,
            NavInsID.RIGHT_CLICK,
            NavInsID.BOTH_CLICK,
            NavInsID.BOTH_CLICK
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    else:
        settings_per_page = 3 if firmware == Firmware.STAX else 2
        instructions = [
            NavInsID.USE_CASE_HOME_SETTINGS,
            NavIns(NavInsID.TOUCH, get_setting_position(firmware, 0, settings_per_page)),
        ]
        navigator.navigate(instructions, screen_change_before_first_instruction=False)
    
     # First we need to get the public key of the device in order to build the transaction    
    file_path = 'samples/apdu/dpath_0.dat'
    apdus = read_lines_from_file(file_path)
    response = backend.exchange_raw(bytes.fromhex(apdus[0])).data
    public_key_x, _ = unpack_get_public_key_response(response)

    # Send the sign tx device instruction.
    # As it requires on-screen validation, the function is asynchronous.
    # It will yield the result when the navigation is done
    file_path = 'samples/apdu/tx_misc_4.dat'
    all_apdus = read_lines_from_file(file_path)

    # send all apdus except last one
    for apdu in all_apdus[:-1]:
        backend.exchange_raw(bytes.fromhex(apdu))

    # send last apdu and yield the response
    with backend.exchange_async_raw(bytes.fromhex(all_apdus[-1])):
        if firmware.device.startswith("nano"):
                navigator.navigate_until_text_and_compare(
                    NavIns(NavInsID.WAIT, (0,)),
                    [
                        NavInsID.BOTH_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.RIGHT_CLICK,
                        NavInsID.BOTH_CLICK
                    ],
                    "Blind",
                    path=ROOT_SCREENSHOT_PATH,
                    test_case_name=test_name
                )
        else:
            navigator.navigate_until_text_and_compare(
                NavIns(NavInsID.WAIT, (0,)),
                [
                    NavInsID.CENTERED_FOOTER_TAP,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.SWIPE_CENTER_TO_LEFT,
                    NavInsID.USE_CASE_REVIEW_CONFIRM,
                    NavInsID.USE_CASE_STATUS_DISMISS
                ],
                "Blind signing ahead",
                path=ROOT_SCREENSHOT_PATH,
                test_case_name=test_name
            )
    
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