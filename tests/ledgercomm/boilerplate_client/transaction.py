from io import BytesIO
from sys import byteorder
from typing import Iterator, Union, Tuple

from boilerplate_client.utils import (read, read_uint, read_varint,
                                      write_varint, UINT64_MAX)


class TransactionError(Exception):
    pass


class Transaction:
    def __init__(self, aa, maxfee, nonce, version, chainid, to, selector, calldata) -> None:

        self.aa = aa
        self.maxfee = maxfee
        self.nonce = nonce
        self.version = version
        self.chainid = chainid
        
        self.to = to
        self.selector = selector
        self.calldata = calldata

    def serialize(self) -> Iterator[Tuple[bool, bytes]]:
        yield False, b"".join([
            # chunk 1 = accountAddress (32 bytes) + maxFee (32 bytes) + nonce (32 bytes) + version (32 bytes) + chain_id (32 bytes)= 160 bytes
            int(self.aa[2:], 16).to_bytes(32, byteorder="big"),
            int(self.maxfee).to_bytes(32, byteorder="big"),
            self.nonce.to_bytes(32, byteorder="big"),
            self.version.to_bytes(32, byteorder="big"),
            int(self.chainid, 16).to_bytes(32, byteorder="big")])

            # chunk 2 = to (32 bytes) + selector length (1 byte) + selector (selector length bytes) + call_data length (1 byte)
        yield False, b"".join([int(self.to[2:], 16).to_bytes(32, byteorder="big"),
            int(len(self.selector)).to_bytes(1, byteorder="big"),
            bytes(self.selector, 'utf-8'),
            int(len(self.calldata)).to_bytes(1, byteorder="big")])

        # chunk n = calldata chunks
        for data in self.calldata:
            if (data[1][:2] == '0x'):
                chunk = b"".join([
                    int(len(data[0])).to_bytes(1, byteorder="big"),
                    data[0].encode('ascii'),
                    int(data[1][2:], 16).to_bytes(32, byteorder="big")])
            else:
                chunk = b"".join([
                    int(len(data[0])).to_bytes(1, byteorder="big"),
                    data[0].encode('ascii'),
                    int(data[1]).to_bytes(32, byteorder="big")])
            
            if (data == self.calldata[-1]):
                yield True, chunk
            else:
                yield False, chunk

    @classmethod
    def from_bytes(cls, hexa: Union[bytes, BytesIO]):
        buf: BytesIO = BytesIO(hexa) if isinstance(hexa, bytes) else hexa

        nonce: int = read_uint(buf, 64, byteorder="big")
        to: bytes = read(buf, 20)
        value: int = read_uint(buf, 64, byteorder="big")
        memo_len: int = read_varint(buf)
        memo: str = read(buf, memo_len).decode("ascii")

        return cls(nonce=nonce, to=to, value=value, memo=memo)
