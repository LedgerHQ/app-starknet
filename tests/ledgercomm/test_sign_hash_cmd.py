from starknet_py.hash.utils import verify_message_signature

bip32_path: str = "m/2645'/1195502025'/1148870696'/0'/0'/0"

# pedersen hashes
hash_0: str = "55b8f28706a5008d3103bcb2bfa6356e56b95c34fed265c955846670a6bb4ef" # 31,5 bytes (63 digits)
hash_1: str = "2bd1d3f8f45a011cbd0674ded291d58985761bbcbc04f4d01c8285d1b35c411" # 31,5 bytes (63 digits)
hash_2: str = "2e672d748fbe3b6e833b61ea8b6e688850247022f06406a1eb83e345ffb417"  # 31 bytes (62 digits)
hash_3: str = "936e8798681b391af0c57fe0bf5703b9631bea18b4bc84b3940ebab234744"   # 30,5 bytes (61 digits) 
hash_4: str = "2534b0f53ccac2347dd51befce72338d9b7c568905f17218d93980ce1fc869f" # 31,5 bytes (63 digits)

hashes = [hash_0, hash_1, hash_2, hash_3, hash_4]

def fix_sign(hash: str) -> str:
    # fix hash to fit into 32 bytes
    while (len(hash) < 63):
        hash = '0' + hash

    assert(len(hash) == 63)
    return hash + '0'
    
def test_sign_hash(cmd, button, model):

    for hash in hashes:

        pub_key_x, pub_key_y = cmd.get_public_key(
            bip32_path=bip32_path,
            display=False
        )

        r, s, v = cmd.sign_hash(bip32_path=bip32_path,
                                hash=bytes.fromhex(fix_sign(hash)),
                                button=button,
                                model=model)
        
        print(int.from_bytes(r, byteorder='big'))
        print(int.from_bytes(s, byteorder='big'))

        assert(
            verify_message_signature(
                msg_hash=int(hash, 16), 
                signature = [int.from_bytes(r, byteorder='big'), int.from_bytes(s, byteorder='big')], 
                public_key=int.from_bytes(pub_key_x, byteorder='big'))
        )