# deploy to LocalTerra

from terra_sdk.client.localterra import LocalTerra
from terra_sdk.util.contract import get_code_id, get_contract_address, read_file_as_b64

lt = LocalTerra()
deploy = lt.wallets["test1"]

print(deployer.key.mnemonic)