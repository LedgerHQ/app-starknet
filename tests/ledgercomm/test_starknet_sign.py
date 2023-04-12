from starkware.starknet.public.abi import get_selector_from_name
from starkware.crypto.signature.fast_pedersen_hash import pedersen_hash
from starkware.starknet.core.os.transaction_hash.transaction_hash import (
    TransactionHashPrefix,
    calculate_transaction_hash_common,
)
from starkware.cairo.common.hash_state import compute_hash_on_elements
from starkware.crypto.signature.signature import private_to_stark_key, sign, verify

signer_address = int("0x7e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a", 16)
contract_address = int("0x0507446de5cfcb833d4e786f3a0510deb2429ae753741a836a7efa80c9c747cb", 16)
selector = get_selector_from_name('mint')
chain_id = int("0x534e5f474f45524c49", 16)
max_fee = int("1000000000000000")
version = 1
nonce = 1
calldata=[int("0x7e00d496e324876bbc8531f2d9a82bf154d1a04a50218ee74cdd372f75a551a", 16), int(1000)]

data_offset = 0
data_len = len(calldata)
call_entry = [contract_address, selector, data_offset, data_len]
call_array_len = 1
wrapped_method_calldata = [call_array_len, *call_entry, len(calldata), *calldata]

private_key = int("0x0115c23c57d28b9d47d32218dbdf200ee0de31149dfee3320f7e0614f1f64c33", 16)

public_key = private_to_stark_key(private_key)

hash_value = calculate_transaction_hash_common(
        tx_hash_prefix=TransactionHashPrefix.INVOKE,
        version=version,
        contract_address=signer_address,
        entry_point_selector=0,
        calldata=wrapped_method_calldata,
        max_fee=max_fee,
        chain_id=chain_id,
        additional_data=[nonce],
)

r, s = sign(msg_hash=hash_value, priv_key=private_key)

assert(verify(msg_hash=hash_value, r=r, s=s, public_key=public_key))
