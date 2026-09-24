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
