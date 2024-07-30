from application_client.response_unpacker import unpack_get_version_response
import toml

# In this test we check the behavior of the device when asked to provide the app version
def test_version(backend):
    # Send the GET_VERSION instruction
    rapdu = backend.exchange_raw(bytes.fromhex("5a00000000"))
    # Read version from Cargo.toml
    with open('starknet/Cargo.toml', 'r') as f:
        config = toml.load(f)
        version = config['package']['version']
        major, minor, patch = version.split('.')
    # Use an helper to parse the response, assert the values
    assert unpack_get_version_response(rapdu.data) == (int(major), int(minor), int(patch))