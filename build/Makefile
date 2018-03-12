# Prerequisites: on Ubuntu 16.04
# Add to /etc/apt/source.d/docker.list
#   deb https://apt.dockerproject.org/repo ubuntu-xenial main
# sudo apt-get update
# sudo apt-get install docker-engine
# sudo usermod -aG docker $USER
# sudo su - $USER - or logout, login.

RUST_TARBALL=rust-nightly-x86_64-unknown-linux-gnu.tar.gz

default: ubuntu

all: centos ubuntu

downloads:
	if [ ! -e files/rust.tar.gz ]; then \
		wget -c https://static.rust-lang.org/dist/$(RUST_TARBALL) -O files/rust.tar.gz; \
	fi

centos: centos6 centos7

centos6: downloads
	cat Dockerfile.centos6 > Dockerfile && \
	echo >> Dockerfile && \
	cat Dockerfile.common >> Dockerfile && \
	docker build --tag=ned-centos6 .
	rm Dockerfile
	docker run -it -v /home/nev/dev/ned/build/bin:/root/bin ned-centos6
	cp bin/ned bin/ned.centos6

centos7: downloads
	cat Dockerfile.centos7 > Dockerfile && \
	echo >> Dockerfile && \
	cat Dockerfile.common >> Dockerfile && \
	docker build --tag=ned-centos7 .
	rm Dockerfile
	docker run -it -v /home/nev/dev/ned/build/bin:/root/bin ned-centos7
	cp bin/ned bin/ned.centos7

ubuntu: downloads
	cat Dockerfile.ubuntu > Dockerfile && \
	echo >> Dockerfile && \
	cat Dockerfile.common >> Dockerfile && \
	docker build --tag=ned-ubuntu .
	rm Dockerfile
	docker run -it -v /home/nev/dev/ned/build/bin:/root/bin ned-ubuntu
	cp bin/ned bin/ned.ubuntu

.PHONY: deploy
deploy:
	./deploy # Not source controlled.

clean:
	rm -f files/rust.tar.gz