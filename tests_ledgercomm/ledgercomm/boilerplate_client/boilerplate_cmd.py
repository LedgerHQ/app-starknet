import struct
import time
from typing import Tuple

from ledgercomm import Transport

from boilerplate_client.boilerplate_cmd_builder import BoilerplateCommandBuilder, InsType
from boilerplate_client.button import Button
from boilerplate_client.exception import DeviceException
from boilerplate_client.transaction import Transaction


class BoilerplateCommand:
    def __init__(self,
                 transport: Transport,
                 debug: bool = False) -> None:
        self.transport = transport
        self.builder = BoilerplateCommandBuilder(debug=debug)
        self.debug = debug

    def get_app_and_version(self) -> Tuple[str, str]:
        sw, response = self.transport.exchange_raw(
            self.builder.get_app_and_version()
        )  # type: int, bytes

        if sw != 0x9000:
            raise DeviceException(error_code=sw, ins=0x01)

        # response = format_id (1) ||
        #            app_name_len (1) ||
        #            app_name (var) ||
        #            version_len (1) ||
        #            version (var) ||
        offset: int = 0

        format_id: int = response[offset]
        offset += 1
        app_name_len: int = response[offset]
        offset += 1
        app_name: str = response[offset:offset + app_name_len].decode("ascii")
        offset += app_name_len
        version_len: int = response[offset]
        offset += 1
        version: str = response[offset:offset + version_len].decode("ascii")
        offset += version_len

        return app_name, version

    def get_version(self) -> Tuple[int, int, int]:
        sw, response = self.transport.exchange_raw(
            self.builder.get_version()
        )  # type: int, bytes

        if sw != 0x9000:
            raise DeviceException(error_code=sw, ins=InsType.INS_GET_VERSION)

        # response = MAJOR (1) || MINOR (1) || PATCH (1)
        assert len(response) == 3

        major, minor, patch = struct.unpack(
            "BBB",
            response
        )  # type: int, int, int

        return major, minor, patch

    def get_app_name(self) -> str:
        sw, response = self.transport.exchange_raw(
            self.builder.get_app_name()
        )  # type: int, bytes

        if sw != 0x9000:
            raise DeviceException(error_code=sw, ins=InsType.INS_GET_APP_NAME)

        return response.decode("ascii")

    def get_public_key(self, bip32_path: str, display: bool = False) -> Tuple[bytes, bytes]:
        sw, response = self.transport.exchange_raw(
            self.builder.get_public_key(bip32_path=bip32_path,
                                        display=display)
        )  # type: int, bytes

        if sw != 0x9000:
            raise DeviceException(error_code=sw, ins=InsType.INS_GET_PUBLIC_KEY)

        # response = pub_key_len (1) ||
        #            pub_key (var) ||

        offset: int = 0
        offset += 1
        pub_key_x: bytes = response[offset:offset + 32]
        offset += 1 + 32
        pub_key_y: bytes = response[offset:offset + 32]

        assert len(response) == 65

        return pub_key_x, pub_key_y

    def sign_hash(self, bip32_path: str, hash: bytes, button: Button, model: str) -> Tuple[bytes, bytes, int]:
        sw: int
        response: bytes = b""

        for chunk in self.builder.sign_hash(bip32_path=bip32_path, hash=hash):
            self.transport.send_raw(chunk)

            sw, response = self.transport.recv() 

            if sw != 0x9000:
                raise DeviceException(error_code=sw, ins=InsType.INS_SIGN_TX)

        # response = sig_len (1) ||
        #            sig (var) ||
        #            v (1)
        offset: int = 0
        sig_len: int = response[offset]
        offset += 1
        r: bytes = response[offset:offset + 32]
        offset += 32
        s: bytes = response[offset:offset + 32]
        offset += 32
        v: int = response[offset]
        offset += 1

        assert len(response) == 1 + sig_len 

        return r, s, v

    def compute_pedersen(self, a: bytes, b: bytes, nb: int, version: int) -> bytes:

        chunk = self.builder.pedersen(a=a, b=b, nb=nb, version=version)

        self.transport.send_raw(chunk)

        sw, response = self.transport.recv()

        if sw != 0x9000:
            raise DeviceException(error_code=sw, ins=InsType.INS_COMPUTE_PEDERSEN)

        h: bytes = response[0:32]

        return h

    def sign_tx(self, bip32_path: str, transaction: Transaction, button: Button, model: str) -> Tuple[int, bytes]:
        sw: int
        response: bytes = b""

        for is_last, chunk in self.builder.sign_tx(bip32_path=bip32_path, transaction=transaction):
            self.transport.send_raw(chunk)

            if is_last:
                time.sleep(2)

                # Review Transaction
                button.right_click()
                # Address
                # Due to screen size, NanoS needs 2 more screens to display the address
                if model == 'nanos':
                    button.right_click()
                    button.right_click()
                button.right_click()
                button.right_click()
                # To
                button.right_click()
                button.right_click()
                if model == 'nanos':
                    button.right_click()
                    button.right_click()
                # Selector
                button.right_click()
                # Calldata #1
                button.right_click()
                button.right_click()
                # Calldata #2
                button.right_click()
                # Approve
                button.both_click()

            sw, response = self.transport.recv()  # type: int, bytes

            if sw != 0x9000:
                raise DeviceException(error_code=sw, ins=InsType.INS_SIGN_TX)

        offset: int = 0
        sig_len: int = response[offset]
        offset += 1
        r: bytes = response[offset:offset + 32]
        offset += 32
        s: bytes = response[offset:offset + 32]
        offset += 32
        v: int = response[offset]
        offset += 1

        assert len(response) == 1 + sig_len 

        return r, s, v
