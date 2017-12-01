#!/bin/bash

#echo "args: ip port"
IP=$1
PORT=$2
if [ ! -n "$IP" ]; then
    IP="127.0.0.1"
fi
if [ ! -n "$PORT" ]; then
    PORT=1337
fi

assert() {
    if [ $1 -ne $2 ]; then
        echo "$3 test failed"
        ./calki_stop.sh
        exit 1
    fi
    echo "$3 test ok"
}

assert_null() {
    if [ "$1" != "null" ]; then
        echo "$2 test failed"
        ./calki_stop.sh
        exit 1
    fi
    echo "$2 test ok"
}

invalid_http_method=-32600
invalid_params=-32605
invalid_data=-32600
invalid_jsonrpc_method=-32601

./calki_start.sh
## request of invalid http method
err_code=`curl -s -X GET -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":2}' $IP:$PORT | jq ".error.code"`
assert $err_code $invalid_http_method "request of invalid http method"

## invalid request of missing id
err_code=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[]}' $IP:$PORT | jq ".error.code"`
assert $err_code $invalid_params "invalid request of missing id"

## request of invalid json
err_code=`curl -s -X POST -d 'abc' $IP:$PORT | jq ".error.code"`
assert $err_code $invalid_params "request of invalid json"

## invalid jsonrpc method
err_code=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"invalid_method","params":[],"id":2}' $IP:$PORT | jq ".error.code"`
assert $err_code $invalid_jsonrpc_method "invalid jsonrpc method"

## invalid request of missing params
err_code=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"calki_getTransaction","params":[],"id":2}' $IP:$PORT | jq ".error.code"`
assert $err_code $invalid_params "invalid request params"

## get null block
result=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"calki_getBlockByHeight","params":[9999999999999999,true],"id":2}' $IP:$PORT | jq ".error.code"`
assert_null $result "get null block"

## get null transaction
result=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"calki_getTransaction","params":["0000000000000000000000000000000000000000000000000000000000000000"],"id":2}' $IP:$PORT | jq ".error.code"`
assert_null $result "get null transaction"

## null block hash
result=`curl -s -X POST -d '{"jsonrpc":"2.0","method":"calki_getBlockByHash","params":["0000000000000000000000000000000000000000000000000000000000000000",true],"id":2}' $IP:$PORT | jq ".error.code"`
assert_null $result "null block hash"

./calki_stop.sh
