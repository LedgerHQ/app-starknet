from enum import IntEnum
from typing import Generator, List, Optional
from contextlib import contextmanager

from ragger.backend.interface import BackendInterface, RAPDU

def bip32_path_from_string(path: str) -> List[bytes]:
    splitted_path: List[str] = path.split("/")

    if not splitted_path:
        raise Exception(f"BIP32 path format error: '{path}'")

    if "m" in splitted_path and splitted_path[0] == "m":
        splitted_path = splitted_path[1:]

    

    return [int(p).to_bytes(4, byteorder="big") if "'" not in p
            else (0x80000000 | int(p[:-1])).to_bytes(4, byteorder="big")
            for p in splitted_path]

MAX_APDU_LEN: int = 255

CLA: int = 0x5A

class InsType(IntEnum):
    GET_VERSION    = 0x00
    GET_PUBLIC_KEY = 0x01
    SIGN_HASH      = 0x02

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

class CommandSender:
    def __init__(self, backend: BackendInterface) -> None:
        self.backend = backend


    def get_app_and_version(self) -> RAPDU:
        return self.backend.exchange(cla=0xB0,  # specific CLA for BOLOS
                                     ins=0x01,  # specific INS for get_app_and_version
                                     p1=0x00,
                                     p2=0x00,
                                     data=b"")


    def get_version(self) -> RAPDU:
        return self.backend.exchange(cla=CLA,
                                     ins=InsType.GET_VERSION,
                                     p1=0x00,
                                     p2=0x00,
                                     data=b"")


    #def get_app_name(self) -> RAPDU:
    #    return self.backend.exchange(cla=CLA,
    #                                 ins=InsType.GET_APP_NAME,
    #                                 p1=P1.P1_START,
    #                                 p2=P2.P2_LAST,
    #                                 data=b"")


    def get_public_key(self, path: str) -> RAPDU:
        bip32_paths: List[bytes] = bip32_path_from_string(path)
        cdata: bytes = b"".join([
            *bip32_paths
        ])
        return self.backend.exchange(cla=CLA,
                                     ins=InsType.GET_PUBLIC_KEY,
                                     p1=0x00,
                                     p2=0x00,
                                     data=cdata)


    @contextmanager
    def get_public_key_with_confirmation(self, path: str) -> Generator[None, None, None]:
        bip32_paths: List[bytes] = bip32_path_from_string(path)
        cdata: bytes = b"".join([
            *bip32_paths
        ])
        with self.backend.exchange_async(cla=CLA,
                                         ins=InsType.GET_PUBLIC_KEY,
                                         p1=0x01,
                                         p2=0x00,
                                         data=cdata) as response:
            yield response


    @contextmanager
    def sign_hash(self, path: str, hash: bytes) -> Generator[None, None, None]:
        bip32_paths: List[bytes] = bip32_path_from_string(path)
        cdata: bytes = b"".join([
            *bip32_paths
        ])
        self.backend.exchange(cla=CLA,
                              ins=InsType.SIGN_HASH,
                              p1=0x00,
                              p2=0x01,
                              data=cdata)

        with self.backend.exchange_async(cla=CLA,
                                         ins=InsType.SIGN_HASH,
                                         p1=0x01,
                                         p2=0x01,
                                         data=hash) as response:
              yield response

    def get_async_response(self) -> Optional[RAPDU]:
        return self.backend.last_async_response
