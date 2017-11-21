#!/bin/bash
set -e

sudo(){
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo -H $*
    fi
    set +o noglob
}
# 1) install add-apt-repository
sudo apt-get update -q
sudo apt-get install -y software-properties-common

# 2) add repositories
# 2.1) add libsodium repository if using trusty version; only for travis trusty build environment.
if [ $(lsb_release -s -c) = "trusty" ]; then
    sudo add-apt-repository -y ppa:chris-lea/libsodium
fi;
# 2.2) add ethereum repository
sudo add-apt-repository -y ppa:ethereum/ethereum

# 3) install runtime dependencies
sudo apt-get update -q
sudo apt-get install -y libstdc++6 rabbitmq-server libssl-dev libgoogle-perftools4 python-pip wget solc libsodium*

# 4) install python package
umask 022
sudo pip install ethereum==2.0.4 pysodium

# 5) extra
# 5.1) libgmssl
wget https://github.com/zibbit/GmSSL/releases/download/v1.0/libgmssl.so.1.0.0.gz
gzip -d libgmssl.so.1.0.0.gz
sudo mv libgmssl.so.1.0.0 /usr/lib/
sudo ln -srf /usr/lib/libgmssl.so.1.0.0 /usr/lib/libgmssl.so
