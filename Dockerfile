FROM ubuntu:latest
 
MAINTAINER Lezwon Castellino "lezwon@gmail.com"
 
RUN apt-get update && apt-get upgrade -y
RUN apt-get install build-essential git -y
RUN apt-get -y install --force-yes libsnappy1v5 libsnappy-dev  capnproto  libgoogle-perftools-dev libssl-dev  libudev-dev  rabbitmq-server  google-perftools jq

RUN mkdir /Calki
COPY . /Calki
WORKDIR /Calki

RUN make setup
RUN make clean
RUN make debug