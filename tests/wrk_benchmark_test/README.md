

## getBlockNum.sh

Function: Statistics between the height of two blocks (including start and end) and the total number of transactions

* The first parameter: the beginning of the block height
* The second parameter: the end of the block height
* The third parameter: jsonrpc  ip address

E.g:
./getBlockNum.sh 10 20 127.0.0.1


## benchmark.sh

Function: Create account, send transaction, output transaction volume, initial height of incoming chain, time

E.g:

1. Create a contract

./benchmark.sh

2. Send contract transactions, the output volume, the starting height of the chain, the time

./benchmark.sh config_call.json

3. Send deposit transaction, the output volume, the starting height of the chain, the time

./benchmark.sh config_store.json


## setPortDelay.sh

Function: Increase random port setting delay

* Parameter 1: Port
* Parameter 2: how many milliseconds delay
* Parameter 3: how many seconds random tc (delayed, lost, repeated, damaged) once
* Parameter 4: tc set type (delay: delay, loss: packet loss, dup: repeat, corrupt: damage)

E.g:

./setPortDelay.sh 4000 1000 5 delay


note:

When parameter 4 is:

* delay, parameter 2 indicates how many milliseconds delay;
* Loss, the parameter 2 shows how much chances are lost;
* dup, the parameter 2 shows how much the probability of duplication;
* corrupt, the parameter 2 shows how much the probability of loss;

The following shows a 20% chance of losing:

./setPortDelay.sh 4000 20 5 loss


## chain_performance.sh

Function: chain performance test

* The first parameter: 1 | 2 | 3
* The second parameter is how many transactions in a block
* The third parameter: profile start time
* The fourth parameter: profile run time

E.g:

1. Create a contract

./chain_performance.sh 1 10000 0 10

2.Contract trading

./chain_performance.sh 2 10000 0 10

3. Stock trading

./chain_performance.sh 3 10000 0 10
