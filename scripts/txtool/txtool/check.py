#!/usr/bin/env python
# coding=utf-8

# TODO lists all the acceptable parameters
# TODO deal with the external parameters passed over


from jsonrpcclient.http_client import HTTPClient
from url_util import endpoint
# '{"jsonrpc":"2.0","method":"calki_blockNumber","params":[],"id":1}'


def check_calki_status():
    result_status = False
    try:
        url = endpoint()
        response = HTTPClient(url).request("calki_blockNumber", [])
        result_status = response > 0
    except:
        result_status = False
    finally:
        return result_status


if __name__ == '__main__':
    if check_calki_status():
        print "CALKI is on."
    else:
        print "CALKI is not working."
