from application_client.command_sender import CommandSender
from application_client.response_unpacker import unpack_get_version_response

import toml

# In this test we check the behavior of the device when asked to provide the app version
def test_version(backend):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # Send the GET_VERSION instruction
    rapdu = client.get_version()
    # Read version from Cargo.toml
    with open('Cargo.toml', 'r') as f:
        config = toml.load(f)
        version = config['package']['version']
        major, minor, patch = version.split('.')
    # Use an helper to parse the response, assert the values
    assert unpack_get_version_response(rapdu.data) == (int(major), int(minor), int(patch))
