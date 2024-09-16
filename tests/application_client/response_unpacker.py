from typing import Tuple
from struct import unpack
from enum import IntEnum

# remainder, data
def pop_sized_buf_from_buffer(buffer:bytes, size:int) -> Tuple[bytes, bytes]:
    return buffer[size:], buffer[0:size]

# remainder, data_len, data
def pop_size_prefixed_buf_from_buf(buffer:bytes) -> Tuple[bytes, int, bytes]:
    data_len = buffer[0]
    return buffer[1+data_len:], data_len, buffer[1:data_len+1]

# Unpack from response:
# response = app_name (var)
def unpack_get_app_name_response(response: bytes) -> str:
    return response.decode("ascii")

# Unpack from response:
# response = MAJOR (1)
#            MINOR (1)
#            PATCH (1)
def unpack_get_version_response(response: bytes) -> Tuple[int, int, int]:
    assert len(response) == 3
    major, minor, patch = unpack("BBB", response)
    return (major, minor, patch)

# Unpack from response:
# response = format_id (1)
#            app_name_raw_len (1)
#            app_name_raw (var)
#            version_raw_len (1)
#            version_raw (var)
#            unused_len (1)
#            unused (var)
def unpack_get_app_and_version_response(response: bytes) -> Tuple[str, str]:
    response, _ = pop_sized_buf_from_buffer(response, 1)
    response, _, app_name_raw = pop_size_prefixed_buf_from_buf(response)
    response, _, version_raw = pop_size_prefixed_buf_from_buf(response)
    response, _, _ = pop_size_prefixed_buf_from_buf(response)

    assert len(response) == 0

    return app_name_raw.decode("ascii"), version_raw.decode("ascii")

# Unpack from response:
# response = pub_key_format (1)
#            pub_key_x (32)
#            pub_key_y (32)
def unpack_get_public_key_response(response: bytes) -> Tuple[bytes, bytes]:
    response, pub_key_format = pop_sized_buf_from_buffer(response, 1)
    response, pub_key_x = pop_sized_buf_from_buffer(response, 32)
    response, pub_key_y = pop_sized_buf_from_buffer(response, 32)

    assert len(pub_key_x) == 32
    assert len(pub_key_y) == 32
    return pub_key_x, pub_key_y

# Unpack from response:
# response = hash (32)
#            sig_len (1)
#            r (32)
#            s (32)
#            v (1)
def unpack_sign_tx_response(response: bytes) -> Tuple[bytes, bytes, bytes, bytes]:
    response, hash = pop_sized_buf_from_buffer(response, 32)
    response, len = pop_sized_buf_from_buffer(response, 1)
    response, r = pop_sized_buf_from_buffer(response, 32)
    response, s = pop_sized_buf_from_buffer(response, 32)
    response, v = pop_sized_buf_from_buffer(response, 1)

    return hash, r, s, v

# Unpack from response:
# response = sig_len (1)
#            r (32)
#            s (32)
#            v (1)
def unpack_sign_hash_response(response: bytes) -> Tuple[bytes, bytes, bytes]:
    response, len = pop_sized_buf_from_buffer(response, 1)
    response, r = pop_sized_buf_from_buffer(response, 32)
    response, s = pop_sized_buf_from_buffer(response, 32)
    response, v = pop_sized_buf_from_buffer(response, 1)

    return r, s, v

class Errors(IntEnum):
    SW_DENY                    = 0x6E04
    SW_CLA_NOT_SUPPORTED       = 0x6E00
    SW_INS_NOT_SUPPORTED       = 0x6E01
    SW_WRONG_P1P2              = 0x6E02
    SW_WRONG_APDU_LENGTH       = 0x6E03
    SW_WRONG_RESPONSE_LENGTH   = 0xB000
    SW_DISPLAY_BIP32_PATH_FAIL = 0xB001
    SW_DISPLAY_ADDRESS_FAIL    = 0xB002
    SW_DISPLAY_AMOUNT_FAIL     = 0xB003
    SW_WRONG_TX_LENGTH         = 0xB004
    SW_TX_PARSING_FAIL         = 0xB005
    SW_TX_HASH_FAIL            = 0xB006
    SW_BAD_STATE               = 0xB007
    SW_SIGNATURE_FAIL          = 0xB008