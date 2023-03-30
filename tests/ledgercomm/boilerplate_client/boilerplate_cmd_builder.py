import enum
import logging
import struct
from typing import List, Tuple, Union, Iterator, cast

from boilerplate_client.transaction import Transaction
from boilerplate_client.utils import bip32_path_from_string

MAX_APDU_LEN: int = 255

def chunkify(data: bytes, chunk_len: int) -> Iterator[Tuple[bool, bytes]]:
    size: int = len(data)

    if size <= chunk_len:
        yield True, data
        return

    chunk: int = size // chunk_len
    remaining: int = size % chunk_len
    offset: int = 0

    for i in range(chunk):
        yield False, data[offset:offset + chunk_len]
        offset += chunk_len

    if remaining:
        yield True, data[offset:]


class InsType(enum.IntEnum):
    INS_GET_VERSION = 0x00
    INS_GET_PUBLIC_KEY = 0x01
    INS_SIGN_HASH = 0x02
    INS_SIGN_TX = 0x03
    INS_COMPUTE_PEDERSEN = 0x04


class BoilerplateCommandBuilder:
    """APDU command builder for the Boilerplate application.

    Parameters
    ----------
    debug: bool
        Whether you want to see logging or not.

    Attributes
    ----------
    debug: bool
        Whether you want to see logging or not.

    """
    CLA: int = 0x80

    def __init__(self, debug: bool = False):
        """Init constructor."""
        self.debug = debug

    def serialize(self,
                  cla: int,
                  ins: Union[int, enum.IntEnum],
                  p1: int = 0,
                  p2: int = 0,
                  cdata: bytes = b"") -> bytes:
        """Serialize the whole APDU command (header + data).

        Parameters
        ----------
        cla : int
            Instruction class: CLA (1 byte)
        ins : Union[int, IntEnum]
            Instruction code: INS (1 byte)
        p1 : int
            Instruction parameter 1: P1 (1 byte).
        p2 : int
            Instruction parameter 2: P2 (1 byte).
        cdata : bytes
            Bytes of command data.

        Returns
        -------
        bytes
            Bytes of a complete APDU command.

        """
        ins = cast(int, ins.value) if isinstance(ins, enum.IntEnum) else cast(int, ins)

        header: bytes = struct.pack("BBBBB",
                                    cla,
                                    ins,
                                    p1,
                                    p2,
                                    len(cdata))  # add Lc to APDU header

        if self.debug:
            logging.info("header: %s", header.hex())
            logging.info("cdata:  %s", cdata.hex())

        return header + cdata

    def get_app_and_version(self) -> bytes:
        """Command builder for GET_APP_AND_VERSION (builtin in BOLOS SDK).

        Returns
        -------
        bytes
            APDU command for GET_APP_AND_VERSION.

        """
        return self.serialize(cla=0xB0,  # specific CLA for BOLOS
                              ins=0x01,
                              p1=0x00,
                              p2=0x00,
                              cdata=b"")

    def get_version(self) -> bytes:
        """Command builder for GET_VERSION.

        Returns
        -------
        bytes
            APDU command for GET_VERSION.

        """
        return self.serialize(cla=self.CLA,
                              ins=InsType.INS_GET_VERSION,
                              p1=0x00,
                              p2=0x00,
                              cdata=b"")

    def get_app_name(self) -> bytes:
        """Command builder for GET_APP_NAME.

        Returns
        -------
        bytes
            APDU command for GET_APP_NAME.

        """
        return self.serialize(cla=self.CLA,
                              ins=InsType.INS_GET_APP_NAME,
                              p1=0x00,
                              p2=0x00,
                              cdata=b"")

    def get_public_key(self, bip32_path: str, display: bool = False) -> bytes:
        """Command builder for GET_PUBLIC_KEY.

        Parameters
        ----------
        bip32_path: str
            String representation of BIP32 path.
        display : bool
            Whether you want to display the address on the device.

        Returns
        -------
        bytes
            APDU command for GET_PUBLIC_KEY.

        """
        bip32_paths: List[bytes] = bip32_path_from_string(bip32_path)

        cdata: bytes = b"".join([
            *bip32_paths
        ])

        return self.serialize(cla=self.CLA,
                              ins=InsType.INS_GET_PUBLIC_KEY,
                              p1=0x01 if display else 0x00,
                              p2=0x00,
                              cdata=cdata)

    def sign_hash(self, bip32_path: str, hash: bytes) -> Iterator[bytes]:
        """Command builder for INS_SIGN_HASH.

        Parameters
        ----------
        bip32_path : str
            String representation of BIP32 path.
        hash : bytes
            Representation of the hash to be signed.

        Yields
        -------
        bytes
            APDU command chunk for INS_SIGN_HASH.

        """
        bip32_paths: List[bytes] = bip32_path_from_string(bip32_path)

        cdata: bytes = b"".join([
            *bip32_paths
        ])

        yield self.serialize(cla=self.CLA,
                                ins=InsType.INS_SIGN_HASH,
                                p1=0x00,
                                p2=0x00,
                                cdata=cdata)

        yield self.serialize(cla=self.CLA,
                                ins=InsType.INS_SIGN_HASH,
                                p1=0x02,
                                p2=0x00,
                                cdata=hash)

    def pedersen(self, a: bytes, b: bytes, nb: int, version: int) -> bytes:

        cdata: bytes = b"".join([
            a,
            b
        ])

        return self.serialize(cla=self.CLA,
                              ins=InsType.INS_COMPUTE_PEDERSEN,
                              p1=nb,
                              p2=version,
                              cdata=cdata)
            
    def sign_tx(self, bip32_path: str, transaction: Transaction) -> Iterator[Tuple[bool, bytes]]:
        """Command builder for INS_SIGN_TX.

        Parameters
        ----------
        bip32_path : str
            String representation of BIP32 path.
        transaction : Transaction
            Representation of the transaction to be signed.

        Yields
        -------
        bytes
            APDU command chunk for INS_SIGN_TX.

        """
        bip32_paths: List[bytes] = bip32_path_from_string(bip32_path)

        cdata: bytes = b"".join([
            len(bip32_paths).to_bytes(1, byteorder="big"),
            *bip32_paths
        ])

        yield False, self.serialize(cla=self.CLA,
                                    ins=InsType.INS_SIGN_TX,
                                    p1=0x00,
                                    p2=0x80,
                                    cdata=cdata)

        for i, (is_last, chunk) in enumerate(transaction.serialize()):
            if is_last:
                yield True, self.serialize(cla=self.CLA,
                                           ins=InsType.INS_SIGN_TX,
                                           p1=i + 1,
                                           p2=0x00,
                                           cdata=chunk)
            else:
                yield False, self.serialize(cla=self.CLA,
                                            ins=InsType.INS_SIGN_TX,
                                            p1=i + 1,
                                            p2=0x80,
                                            cdata=chunk)
