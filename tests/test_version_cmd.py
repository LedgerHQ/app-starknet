from application_client.command_sender import CommandSender
from application_client.response_unpacker import unpack_get_version_response

# Taken from the Cargo.toml, to update every time the version is bumped
MAJOR = 1
MINOR = 1
PATCH = 1

# In this test we check the behavior of the device when asked to provide the app version
def test_version(backend):
    # Use the app interface instead of raw interface
    client = CommandSender(backend)
    # Send the GET_VERSION instruction
    rapdu = client.get_version()
    # Use an helper to parse the response, assert the values
    assert unpack_get_version_response(rapdu.data) == (MAJOR, MINOR, PATCH)
