FROM centos:latest
COPY rust-nightly-x86_64-unknown-linux-gnu.tar.gz /opt
RUN tar xf /opt/rust-nightly-x86_64-unknown-linux-gnu.tar.gz
RUN rust-nightly-x86_64-unknown-linux-gnu/install.sh
RUN yum install git