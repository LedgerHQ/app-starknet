from pathlib import Path
from hashlib import sha256

import subprocess
import os

ROOT_SCREENSHOT_PATH = Path(__file__).parent.resolve()

def read_lines_from_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    return [line.strip('=> ').rstrip() for line in lines]

def call_external_binary(binary_path, *args):
    try:

        os.chmod(binary_path, 0o755)

        # Call the external binary with arguments
        result = subprocess.run([binary_path, *args], capture_output=True, text=True, check=True)
        
        # Get the standard output and standard error
        stdout = result.stdout.strip()  # Use strip() to remove any leading/trailing whitespace
        stderr = result.stderr
        
        # Return the output and error
        return stdout, stderr
    except subprocess.CalledProcessError as e:
        # Handle errors in the called process
        print(f"Error calling {binary_path}: {e}")
        return None, e.stderr


# Check if a signature of a given message is valid
#def check_signature_validity(public_key: bytes, signature: bytes, message: bytes) -> bool:
#    pk: VerifyingKey = VerifyingKey.from_string(
#        public_key,
#        curve=SECP256k1,
#        hashfunc=sha256
#    )
#    return pk.verify(signature=signature,
#                     data=message,
#                     hashfunc=keccak_256,
#                     sigdecode=sigdecode_der)
