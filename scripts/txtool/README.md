# Help new users of CALKI understand the flow of operations

### Install the required dependencies

`` `
$ sudo add-apt-repository ppa: ethereum / ethereum
$ sudo apt-get update
$ sudo apt-get install solc
`` `

`` `
$ pip install -r requirements.txt
$ bash requirements_sudo.sh
`` `

### Check if CALKI starts up normally
`` `
$ python check.py
`` `

### net_peerCount

`` `
$ python peer_count.py
`` `

### calki_blockNumber

`` `
$ python block_number.py
`` `

### Generate account information (account information is saved in the output / accounts directory)

Use secp256k1 signature algorithm and sha3 hash

`` `
$ python generate_account.py
`` `

Use ed25519 signature algorithm and blake2b hash

`` `
$ python generate_account.py --newcrypto
`` `

### Compile the contract

`` `
The absolute path to the incoming file
$ python compile.py -f /home/jerry/rustproj/calki/admintool/txtool/txtool/tests/test.sol

Or incoming source
$ python compile.py -s "pragma solidity ^ 0.4.0;

contract SimpleStorage {
    uint storedData;
    event Init (address, uint);
    event Set (address, uint);

    function SimpleStorage () {
        storedData = 100;
        Init (msg.sender, 100);
    }

    event Stored (uint);

    function set (uint x) {
        Stored (x);
        storedData = x;
        Set (msg.sender, x);
    }

    function get () constant returns (uint) {
        return storedData;
    }
} "

The result of contract compilation is saved in the output / compiled directory
`` `

Get the address of the function that compiled the contract
`` `
$ python compile.py -p "get ()"
0x6d4ce63c
`` `

### Constructs a transaction

Use secp256k1 signature algorithm and sha3 hash
`` `
$ python make_tx.py

$ python make_tx.py --code `contract bytecode` --privkey` privatekey` --to `transaction to`
`` `
Use ed25519 signature algorithm and blake2b hash

`` `
$ python make_tx.py --newcrypto

$ python make_tx.py --code `contract bytecode` --privkey` privatekey` --to `transaction to` --newcrypto
`` `


### Send the transaction
Transaction related information is stored in the output / transaction directory

`` `
$ python send_tx.py

$ python send_tx.py `deploycode`

$ python send_tx.py --codes `deploycode1` deploycode2` deploycode3` ...
`` `

### Get the deal
The hash of the transaction uses the value in the output / transaction / hash file

`` `
$ python get_tx.py

$ python get_tx.py --tx `transaction_hash`
`` `

### calki_getBlockByHash

`` `
$ python block_by_hash.py hash --detail
$ python block_by_hash.py hash --no-detail
`` `

### calki_getBlockByNumber

`` `
$ python block_by_number.py number --detail
$ python block_by_number.py number --no-detail
`` `

### Get the receipt

`` `
$ python get_receipt.py
$ python get_receipt.py --tx `transaction_hash`
`` `

### eth_getTransactionCount

`` `
$ python tx_count.py `block_number` -a` address`
`` `

### eth_getCode
`` `
$ python get_code.py `address`` number`
`` `
### Get Logs

`` `
$ python get_logs.py
`` `

### Call contract
`` `
$ python call.py `to`` data`

$ python call.py `to`` data`` block_number` --sender` option sender`

to --- contract address
data --- contract method, params encoded data
data construct reference contract ABI
`` `
