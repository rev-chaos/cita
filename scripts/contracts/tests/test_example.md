## test example contract using cita solidity

0. 部署合约，发送者需要创建合约权限


```
python make_tx.py --privkey "352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214" --code "606060405234156100105760006000fd5b610015565b60e0806100236000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604b5780636d4ce63c14606c576045565b60006000fd5b341560565760006000fd5b606a60048080359060200190919050506093565b005b341560775760006000fd5b607d60a3565b6040518082815260200191505060405180910390f35b8060006000508190909055505b50565b6000600060005054905060b1565b905600a165627a7a72305820942223976c6dd48a3aa1d4749f45ad270915cfacd9c0bf3583c018d4c86f9da20029"
```

*详见README中步骤1*

* `code`: 为`solc test_out_of_quota --bin`得到

2. run `python send_tx.py`

结果如下:

```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0x61854d356645ab5aacd24616e59d76ac639c5a5c2ec79292f8e8fb409b42177b","status":"Ok"}} 
```

*详见README中步骤2*

3. run `python get_receipt.py`

结果如下:

```
{
  "contractAddress": "0x73552bc4e960a1d53013b40074569ea05b950b4d",
  "cumulativeGasUsed": "0xafc8",
  "logs": [],
  "blockHash": "0x14a311d9f026ab592e6d156f2ac6244b153816eeec18717802ee9e675f0bfbbd",
  "transactionHash": "0x61854d356645ab5aacd24616e59d76ac639c5a5c2ec79292f8e8fb409b42177b",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x6",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0xafc8"
}
```

其中:
* `contractAddress`: 为部署合约的地址，即`0x73552bc4e960a1d53013b40074569ea05b950b4d`

4. get function hash

run `solc example.sol --hash`，结果如下:

```
======= example.sol:SimpleStorage =======
Function signatures: 
6d4ce63c: get()
60fe47b1: set(uint256)
```

其中:
* `get`: hash is `0x6d4ce63c`
* `set`: hash is `0x60fe47b1`

5. 调用合约 set function

*set the num: 1*

```
python make_tx.py --privkey "352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214" --to "73552bc4e960a1d53013b40074569ea05b950b4d" --code "60fe47b10000000000000000000000000000000000000000000000000000000000000001"
```

*详见README中步骤1*

6. run `python sen_tx.py`

```
{"jsonrpc":"2.0","id":1,"result":{"hash":"0xf29935d0221cd8ef2cb6a265e0a963ca172aca4f6e43728d2ccae3127631d590","status":"Ok"}}
```

*详见README中步骤2*

7. run `python get_receipt.py`

```
{
  "contractAddress": null,
  "cumulativeGasUsed": "0x4f2d",
  "logs": [],
  "blockHash": "0x2a10ae38be9e1816487dbfb34bce7f440d60035e8978146caef5d14608bb222c",
  "transactionHash": "0xf29935d0221cd8ef2cb6a265e0a963ca172aca4f6e43728d2ccae3127631d590",
  "root": null,
  "errorMessage": null,
  "blockNumber": "0x15",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "transactionIndex": "0x0",
  "gasUsed": "0x4f2d"
}
```

*详见README中步骤2*

8. use `eth_call` to call the get funciton

```
curl -X POST --data '{"jsonrpc":"2.0","method":"eth_call", "params":[{"to":"0x73552bc4e960a1d53013b40074569ea05b950b4d", "data":"0x6d4ce63c"}, "latest"],"id":2}' 127.0.0.1:1337
```

结果：

```
{"jsonrpc":"2.0","id":2,"result":"0x0000000000000000000000000000000000000000000000000000000000000001"}
```

*check the result: 1*