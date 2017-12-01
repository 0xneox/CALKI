## Run the script

1. Switch directory, run the following command:

`` `
cd calki / tests / jsonrpc_performance
`` `

2. Run the following command:

`` `
../../target/install/bin/jsonrpc_performance --config config_err_format.json
`` `

Which `config_err_format.json` is a request in the wrong format, other requests are similar to the following:
* config_correct.json (request in the correct format)
* config_get_height.json (get height request)
* config_dup.json (repeat transaction)
* config_signerr.json (verification error)

## Test Results

The output is as follows:

`` `
20171011 09:13:55 ~ 09:28:27 - INFO - test type: jsonrpc + auth + consensus (corrent), tx_num: 200000, start_h: 2719, end_h: 2952, jsonrpc use time: 849452 ms, tps: 235
`` `

among them:

* tx_num: Number of transactions sent
* start_h: start height
* end_h: end of the height
* jsonrpc use time: Spent time (ms)
