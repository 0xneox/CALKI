#!/usr/bin/env python
# coding=utf-8

from jsonrpcclient.http_client import HTTPClient
from url_util import host, endpoint


def block_number():
    try:
        url = endpoint()
        response = HTTPClient(url).request("calki_blockNumber", [])
    except:
        return None

    return response


def main():
    number = block_number()
    print int(number, 16)

if __name__ == "__main__":
    main()
