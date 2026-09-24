from application_client.response_unpacker import Errors
from ragger.backend import RaisePolicy
from utils import read_lines_from_file

# A transaction reaches the device as a fixed sequence of APDUs: P1=0 sets the
# derivation path, every following P1 is sent once and in order, and only the
# last one repeats to stream calls or constructor calldata. The device hashes
# what it receives as it goes and builds the review from what it parsed, so any
# APDU outside that sequence could put felts in the signed hash that the review
# never accounts for. These tests send such sequences and check that each one
# is refused before it can reach a review, and that the request is dropped.

CLA = 0x5A
INS_SIGN_TX = 0x03
INS_SIGN_TX_V1 = 0x04
INS_SIGN_DEPLOY_ACCOUNT = 0x05

# Stark field prime
P = 0x800000000000011000000000000000000000000000000000000000000000001

# Felts a malicious host would slip into the signed calldata: a call target,
# a selector and a calldata length, none of which the review would show.
HIDDEN_CALL = (bytes.fromhex("0" * 63 + "1") * 2) + (3).to_bytes(32, "big")


def load(sample_name):
    return [bytes.fromhex(a) for a in read_lines_from_file(f'samples/apdu/{sample_name}.dat')]


def apdu(ins, p1, p2, data):
    return bytes([CLA, ins, p1, p2, len(data)]) + data


def send(backend, raw):
    return backend.exchange_raw(raw).status


def send_all(backend, apdus):
    for a in apdus:
        assert send(backend, a) == 0x9000


def assert_dropped(backend, raw, status=Errors.SW_TX_BAD_STATE):
    assert send(backend, raw) == status


def start(backend):
    backend.raise_policy = RaisePolicy.RAISE_NOTHING
    send_all(backend, load('dpath_0')[:1])


# A continuation chunk (P2=1) before any call has started would be hashed
# without belonging to the call the review is built from.
def test_invoke_v3_add_before_new_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:6])
    assert_dropped(backend, apdu(INS_SIGN_TX, 6, 1, HIDDEN_CALL))
    # The request is gone: the benign call that would follow is refused too.
    assert_dropped(backend, apdus[6])


def test_invoke_v1_add_before_new_refused(backend):
    start(backend)
    apdus = load('tx_v1_transfer_ETH_0')
    send_all(backend, apdus[:3])
    assert_dropped(backend, apdu(INS_SIGN_TX_V1, 3, 1, HIDDEN_CALL))
    assert_dropped(backend, apdus[3])


# A new call cannot start while the previous one still expects calldata.
def test_invoke_v3_new_call_before_previous_complete_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:6])
    call = apdus[6]
    # to, selector, calldata length 3, then only the first calldata felt
    partial = call[5:5 + 4 * 32]
    assert send(backend, apdu(INS_SIGN_TX, 6, 0, partial)) == 0x9000
    assert_dropped(backend, call)


# A non-canonical felt (>= P) hashes like its reduced value but would be read
# as another value wherever the app interprets it.
def test_invoke_v3_non_canonical_calldata_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:6])
    call = bytearray(apdus[6][5:])
    # recipient + P
    recipient = int.from_bytes(call[96:128], "big") + P
    call[96:128] = recipient.to_bytes(32, "big")
    assert_dropped(backend, apdu(INS_SIGN_TX, 6, 0, bytes(call)))


# Replaying the call-count stage would hash an extra felt per replay
# while keeping only the last count.
def test_invoke_v3_call_count_replay_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:6])
    assert_dropped(backend, apdus[5])
    assert_dropped(backend, apdus[6])


def test_invoke_v1_call_count_replay_refused(backend):
    start(backend)
    apdus = load('tx_v1_transfer_ETH_0')
    send_all(backend, apdus[:3])
    assert_dropped(backend, apdus[2])
    assert_dropped(backend, apdus[3])


# A call count that does not fit in 32 bits would be hashed whole but read
# truncated.
def test_invoke_v3_oversized_call_count_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:5])
    assert_dropped(backend, apdu(INS_SIGN_TX, 5, 0, ((1 << 32) + 1).to_bytes(32, "big")))


def test_stage_skipped_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:1])
    # fees (P1=2) before the transaction fields (P1=1)
    assert_dropped(backend, apdus[2])


def test_cross_ins_continuation_refused(backend):
    start(backend)
    send_all(backend, load('tx_v3_transfer_ETH')[:2])
    # an Invoke V1 stage on top of an Invoke V3 request
    assert_dropped(backend, load('tx_v1_transfer_ETH_0')[2])


def fee_words(fee_apdu):
    data = fee_apdu[5:]
    return [data[i:i + 32] for i in range(0, len(data), 32)]


def fee_apdu(ins, words):
    return apdu(ins, 2, 0, b"".join(words))


# A non-canonical bound x + P hashes as x but its bytes decode to other
# amounts and prices, so the review would show a fee unrelated to the signed one.
def test_fees_non_canonical_bound_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:2])
    words = fee_words(apdus[2])
    words[2] = (int.from_bytes(words[2], "big") + P).to_bytes(32, "big")
    assert_dropped(backend, fee_apdu(INS_SIGN_TX, words), Errors.SW_INVALID_FEES)


def test_fees_wrong_resource_order_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:2])
    words = fee_words(apdus[2])
    words[1], words[2] = words[2], words[1]
    assert_dropped(backend, fee_apdu(INS_SIGN_TX, words), Errors.SW_INVALID_FEES)


def test_fees_oversized_tip_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:2])
    words = fee_words(apdus[2])
    words[0] = (1 << 64).to_bytes(32, "big")
    assert_dropped(backend, fee_apdu(INS_SIGN_TX, words), Errors.SW_INVALID_FEES)


def test_fees_truncated_refused(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH')
    send_all(backend, apdus[:2])
    assert_dropped(backend, fee_apdu(INS_SIGN_TX, fee_words(apdus[2])[:2]),
                   Errors.SW_INVALID_FEES)


def test_deploy_account_fees_non_canonical_bound_refused(backend):
    start(backend)
    apdus = load('tx_v3_deploy_account')
    send_all(backend, apdus[:2])
    words = fee_words(apdus[2])
    words[1] = (int.from_bytes(words[1], "big") + P).to_bytes(32, "big")
    assert_dropped(backend, fee_apdu(INS_SIGN_DEPLOY_ACCOUNT, words), Errors.SW_INVALID_FEES)


# Once a transaction has been reviewed, whatever the outcome, its last APDU
# cannot bring the review back.
def test_completed_request_cannot_be_replayed(backend):
    start(backend)
    apdus = load('tx_v3_transfer_ETH_extra_calldata')
    send_all(backend, apdus[:-1])
    with backend.exchange_async_raw(apdus[-1]):
        backend.wait_for_screen_change()
    assert backend.last_async_response.status == Errors.SW_MALFORMED_TRANSFER_CALLDATA
    assert_dropped(backend, apdus[-1])
