FROM ubuntu:latest
COPY rust-nightly-x86_64-unknown-linux-gnu.tar.gz /opt
RUN tar xf /opt/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
RUN rust-nightly-x86_64-unknown-linux-gnu/install.sh
RUN apt-get install software-properties-common
RUN add-apt-repository ppa:git-core/ppa
RUN apt-get update
RUN apt-get install git