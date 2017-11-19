#CALKI instructions

## JSON-RPC

* net_peerCount
* calki_blockNumber
* calki_sendTransaction
* calki_getBlockByHash
* calki_getBlockByNumber
* calki_getTransaction
* eth_getTransactionCount
* eth_getCode
* eth_getTransactionReceipt
* eth_call

***
#### net_peerCount

The current number of node connections.

##### Parameters
none

##### Returns
QUANTITY - integer of the number of connected peers.

##### Example
`` `js
// Request
curl -X POST --data '{"jsonrpc": "2.0", "method": "net_peerCount", "params": [], "id": 74}'

// Result
{
    "id": 74,
    "jsonrpc": "2.0",
    "result": "0x3"
}
`` `
***

#### calki_blockNumber

Returns the current block height.

##### Parameters
none

##### Returns

`QUANTITY` - integer of current block height of CALKI.

##### Example
`` `js
// Request
curl -X POST --data '{"jsonrpc": "2.0", "method": "calki_blockNumber", "params": [], "id": 83}'

// Result
{
    "id": 83,
    "jsonrpc": "2.0",
    "result": "0x1d10"
}
`` `

***
#### calki_sendTransaction

Call the blockchain interface through a serialized transaction.

##### Parameters

1. `DATA`, The signed transaction data.
`` `js
const signed_data = "0a9b0412013018fface20420f73b2a8d046060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6 000805490505b905600a165627a7a72305820631927ec00e7a86b68950c2304ba2614a8dcb84780b339fc2bfe442bba418ce800291241884bfdfd8e417ab286fd761d42b71a9544071d91084c56f9063471ce82e266122a8f9a24614e1cf75070eea301bf1e7a65857def86093b6892e09ae7d0bcdff901 "
params: [signed_data]
`` `

### The process of generating signature data
#### Constructs a protobuf data structure
`` `js
// Transaction
syntax = "proto3";
enum Crypto {
    SECP = 0;
    SM2 = 1;
}

// nonce identifies the uniqueness of the transaction
message transaction {
    string to = 1;
    string nonce = 2;
    uint64 quota = 3; // gas
    uint64 valid_until_block = 4;
    bytes data = 5;
}

message UnverifiedTransaction {
    Transaction transaction = 1;
    bytes signature = 2;
    Crypto crypto = 3;
}
`` `
####? Obtain the bytecode corresponding to the contract

The following code fragment for the sample code, the specific method for obtaining the contract bytecode reference [document] (https://ethereum.stackexchange.com/questions/8115/how-to-get-the-bytecode-of-a-transaction-using- the-solidity-browser)

[solidity] (https://solidity.readthedocs.io/en/develop/) Related documents
`` `
pragma solidity ^ 0.4.15;

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
}

`` `
####? Construct a signature

1. Construct Transaction object tx, fill to, nonce, valid_until_block, data 4 fields.
2. tx object protobuf serialized sha3 -> hash
Sign the hash -> signature
4. Construct UnverifiedTransaction, fill it with hash, signature, SECP UnverifiedTransaction -> unverify_tx
5. unverify_tx object protobuf serialization

Pseudocode Description:

`` `
let tx = Transaction :: new ();
// contract bytecode
let data = "6060604052341561000f57600080fd5b5b60646000819055507f8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b5b610178806100956000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b1146100495780636d4ce63c1461006c575b600080fd5b341561005457600080fd5b61006a6004808035906020019091905050610095565b005b341561007757600080fd5b61007f610142565b6040518082815260200191505060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507ffd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b93382604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018281526020019250505060405180910390a15b50565b6000805490505b905600a165627a7a723058207fbd8b From_hex ();
tx.setdata (data);
if not depoly_contract {
    tx.setTo (address);
}
tx.set_valid_until_block (9999999);
tx.set_nonce (nonce);

// language_depend_method and sign are the way to handle private keys and signatures in the appropriate language or library
let privkey = language_depend_method ("966fc50326cf6e2b30b06d82147
