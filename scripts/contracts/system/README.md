# Test system contract functions and repeat transactions

## Test System Contract Features

Temporarily include Consensus Node Management Contract, Entitlement Management Contract and Quota Management Contract

Test procedure (script):
Calling eth_call Get get class function return value (limited to the system contract because its address is fixed)
1. Call the tests / integrate_test / calki_start.sh script to start CALKI
Called scripts / txtool / txtool / check.py to verify CALKI startup
3. Call scripts / txtool / txtool / txtool / make_tx.py to construct the transaction
4. Call scripts / txtool / txtool / txtool / send_tx to send the transaction
5. Calls scripts / txtool / txtool / txtool / get_receipt to get the result validation in the receipt
6. Call eth_call Get function return value
The above is a common test process, the actual test process requires multiple calls 3-6 process

* The above txtool script usage can view its README document *

### Consensus Node Management

#### Test Function 1: Add Consensus Node
* (The process needs to call the permissions system contract to get the hair trading permissions) *

Process and data structure:

0. use jsonrpc to check the list of the consensus node

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337

* Constructor see jsonRPC README file *

RESULT:
```
{"jsonrpc":"2.0","id":2,"result":"0x000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000507f93743cf1dd841b1e9c028a6ed7893c176015dab7f938478e4f80781c0e0c786b2efbbca68a2248e9b5b04c2c10681800c4ff7e1307fd7d4eab7715f12bd7b684f1c958c15548ebbce378726c9cf4d000000000000000000000000000000000"}
```

Result : Get specefic consenus node data need to be parsed (analytical method ) [contract ABI](https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI)

1. Permission given by the address 0x1a702a25c6bca72b67987968f0bfb3a3213c5688 authority to add and delete nodes 0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523
( these two address are built in system )

```
python make_tx.py --to "00000000000000000000000000000000013241a4" --code "301da8700000000000000000000000004b5ae4567ad5d9fb92bc9afd6a657e6fa13a25230000000000000000000000000000000000000000000000000000000000000001" --privkey "866c936ff332228948bdefc15b1877c88e0effce703ee6de898cffcafe9bbe25"
```
Construction method:
* `make_tx.py` detailed usage You can run` pythpn make_tx.py -h` to view
* `--to`: The address of the contract to invoke
* `--code`: Constructor for the contract method hash plus its parameters, refer to [contract ABI] (https://github.com/ethereum/wiki/wiki/Ethereum-Contract-ABI). The contract hash can be obtained from the command `solc x.sol --hash` or via [remix] (remix.ethereum.org)
* `--privkey`: Signature private key representing the transaction. An instance is the private key of an account that has transactional authority

2. `python send_tx.py`. The result is as follows:
```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0xde05dc52e88ff6d3d1ce8212255dd3a13444edbd507c7d401ad1019ba8f75355","status":"Ok"}}
```
The result is as follows:
* `hash`: Represents the generated transaction hash
* `status`: send transaction status. `OK` to send successfully

3. `python get_receipt.py` results as follows:

```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0xbb06",
  "logs": [
    {
      "blockHash": "0xc8fe0a4961bdc67f25c668ec775dd52a4552120a3fb5178600b95f548d778eff",
      "transactionHash": "0xde05dc52e88ff6d3d1ce8212255dd3a13444edbd507c7d401ad1019ba8f75355",
      "transactionIndex": "0x0",
      "topics": [
        "0xca571b17c94502f9fcce67874fd8e4ac41a6139e5dd6f79836393bcc12a0e765"
      ],
      "blockNumber": "0x21",
      "address": "0x00000000000000000000000000000000013241a4",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x0000000000000000000000004b5ae4567ad5d9fb92bc9afd6a657e6fa13a25230000000000000000000000000000000000000000000000000000000000000001"
    }
  ],
  "blockHash": "0xc8fe0a4961bdc67f25c668ec775dd52a4552120a3fb5178600b95f548d778eff",
  "transactionHash": "0xde05dc52e88ff6d3d1ce8212255dd3a13444edbd507c7d401ad1019ba8f75355",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x21",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000004000000000000000000000000000000000000080000000000000000000000000000100000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0xbb06"
}
```
among them:
* `errorMessage`:` null` means the sending was successful

4. Increase the consensus node by the address with the increase / decrease node management. The added address of the test is 0x3f1a71d1d8f073f4e725f57bbe14d67da22f888`. First call the `new` method
`` `
python make_tx.py --to "00000000000000000000000000000000013241a2" --code "ddad2ffe000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
```
5. python send_tx.py

```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0x54d242d3284181610f663a41f1b0c3e14851ae928065f02a03690660a77f1de4","status":"Ok"}} 
```

6. python get_receit.py

```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0x56b0",
  "logs": [
    {
      "blockHash": "0xbe2a2b5c0b307155cdd78f665ecef6c0464f7b86262ed1f04f187c4028bcf32c",
      "transactionHash": "0x54d242d3284181610f663a41f1b0c3e14851ae928065f02a03690660a77f1de4",
      "transactionIndex": "0x0",
      "topics": [
        "0xfd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c1"
      ],
      "blockNumber": "0x37",
      "address": "0x00000000000000000000000000000000013241a2",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888"
    }
  ],
  "blockHash": "0xbe2a2b5c0b307155cdd78f665ecef6c0464f7b86262ed1f04f187c4028bcf32c",
  "transactionHash": "0x54d242d3284181610f663a41f1b0c3e14851ae928065f02a03690660a77f1de4",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x37",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000208000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0x56b0"
}
```
see step 2 
6. python get_receit.py

```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0x56b0",
  "logs": [
    {
      "blockHash": "0xbe2a2b5c0b307155cdd78f665ecef6c0464f7b86262ed1f04f187c4028bcf32c",
      "transactionHash": "0x54d242d3284181610f663a41f1b0c3e14851ae928065f02a03690660a77f1de4",
      "transactionIndex": "0x0",
      "topics": [
        "0xfd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c1"
      ],
      "blockNumber": "0x37",
      "address": "0x00000000000000000000000000000000013241a2",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888"
    }
  ],
  "blockHash": "0xbe2a2b5c0b307155cdd78f665ecef6c0464f7b86262ed1f04f187c4028bcf32c",
  "transactionHash": "0x54d242d3284181610f663a41f1b0c3e14851ae928065f02a03690660a77f1de4",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x37",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000200000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000208000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0x56b0"
}
```

* `data`: The event recorded during the call to the contract method, in contrast to the data when the transaction was constructed. check `0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888` is the test address.

7. Call `approve` to upgrade the test address to a consensus node address

```
python make_tx.py --to "00000000000000000000000000000000013241a2" --code "dd4c97a0000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888" --privkey "5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6"
```
8. python send_tx.py

```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0x2fc76cc95e265bbaf0ded7787d76d34ea02df56f709e2e84168de04b700ad800","status":"Ok"}}
```
9. python get_receipt.py

```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0xced5",
  "logs": [
    {
      "blockHash": "0x0ae7bf0a99b0b4468a718575ebfdd12621898b16df18b113c624faeec0907bbe",
      "transactionHash": "0x2fc76cc95e265bbaf0ded7787d76d34ea02df56f709e2e84168de04b700ad800",
      "transactionIndex": "0x0",
      "topics": [
        "0x5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb"
      ],
      "blockNumber": "0x3d",
      "address": "0x00000000000000000000000000000000013241a2",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888"
    }
  ],
  "blockHash": "0x0ae7bf0a99b0b4468a718575ebfdd12621898b16df18b113c624faeec0907bbe",
  "transactionHash": "0x2fc76cc95e265bbaf0ded7787d76d34ea02df56f709e2e84168de04b700ad800",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x3d",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000010000000000000000040000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0xced5"
}
```

* `data`: The event recorded during the call to the contract method, in contrast to the data when the transaction was constructed. check `0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f888` is the test address.


10. Check chain.log
```
20171011 16:28:59 - TRACE - node ad480135eaf2fe211ea23508b0ad014d9e9ffd35
20171011 16:28:59 - TRACE - node bcbd3a00f2b79e2f3f5763b8b832f64077fd3d52
20171011 16:28:59 - TRACE - node 447cab8c53a5474628b857c5707d9ad9090a1502
20171011 16:28:59 - TRACE - node 6f8c4f89e49d8689712873f0a541a87f75a4772c
20171011 16:28:59 - TRACE - node d3f1a71d1d8f073f4e725f57bbe14d67da22f888
```
*Chain can see the consensus node more `d3f1a71d1d8f073f4e725f57bbe14d67da22f888`

11. check the consensus node list of the genensis 

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337
```

12. check the consensus node list of the new

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x00000000000000000000000000000000013241a2", "data":"0x609df32f"}, "latest"],"id":2}' 127.0.0.1:1337
```
Result:

```
{"jsonrpc":"2.0","id":2,"result":"0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000006436234c42397f76dee7a50880db28965164cfa245cee22b6524d24a9796f29ee8e21f6c65e5326b89e7b5fc36940c1370aaea804353fdfc02a8915bab08666047109571f510abaaab9efeb932bdc3eedfd3f1a71d1d8f073f4e725f57bbe14d67da22f88800000000000000000000000000000000000000000000000000000000"}
```
* `result` Compared with` result` in the above step, one consensus node is added and the node with more than one result is the test address * The file is not available *

## Quota management

#### Test Function 1: call setGlobalAccountGasLimit function
* (The process needs to call the permissions system contract to get the hair trading permissions) *

Process and data structure:

1.The permission to send the transaction is granted by the address 0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888 with the permission address 0x1a702a25c6bca72b67987968f0bfb3a3213c5688 with the addition and deletion of node management
```
python make_tx.py --to "00000000000000000000000000000000013241a4" --code "301da870000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f8880000000000000000000000000000000000000000000000000000000000000001" --privkey "866c936ff332228948bdefc15b1877c88e0effce703ee6de898cffcafe9bbe25"
```
*see step1

2.python send_tx.py . Results:

```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0x19403b61c93731f1c0473b4e22f8fcbd392f6078f360b8b9f44aca870780135d","status":"Ok"}} 
```

*see step2*
3.python get_receipt.py . Results 
```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0xbb4d",
  "logs": [
    {
      "blockHash": "0xb00297f08d90cd12e1dd9582285b432431d799ea23a2daf9abe59739e8ed42ba",
      "transactionHash": "0x19403b61c93731f1c0473b4e22f8fcbd392f6078f360b8b9f44aca870780135d",
      "transactionIndex": "0x0",
      "topics": [
        "0xca571b17c94502f9fcce67874fd8e4ac41a6139e5dd6f79836393bcc12a0e765"
      ],
      "blockNumber": "0x10",
      "address": "0x00000000000000000000000000000000013241a4",
      "transactionLogIndex": "0x0",
      "logIndex": "0x0",
      "data": "0x000000000000000000000000d3f1a71d1d8f073f4e725f57bbe14d67da22f8880000000000000000000000000000000000000000000000000000000000000001"
    }
  ],
  "blockHash": "0xb00297f08d90cd12e1dd9582285b432431d799ea23a2daf9abe59739e8ed42ba",
  "transactionHash": "0x19403b61c93731f1c0473b4e22f8fcbd392f6078f360b8b9f44aca870780135d",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x10",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000004000000000000000000000000000000000000080000000000000000000000000000100000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0xbb4d"
}
```

*see step3*
4. By setting the quota address to set the quota operation, the test set quota `0x2b0ce58`.
```
python make_tx.py --to "00000000000000000000000000000000013241a3" --code "c9bcec770000000000000000000000000000000000000000000000000000000002b0ce58" --privkey "61b760173f6d6b87726a28b93d7fcb4b4f842224921de8fa8e49b983a3388c03"
```






