# Prerequisites: on Ubuntu 16.04
# Add to /etc/apt/source.d/docker.list
#   deb https://apt.dockerproject.org/repo ubuntu-xenial main
# sudo apt-get update
# sudo apt-get install docker-engine
# sudo usermod -aG docker $USER
# sudo su - $USER - or logout, login.

RUST_TARBALL=rust-1.8.0-x86_64-unknown-linux-gnu.tar.gz

default: prereqs centos ubuntu

prereqs:
	if [ ! -e files/rust.tar.gz ]; then \
		wget -c https://static.rust-lang.org/dist/$(RUST_TARBALL); \
		mv $(RUST_TARBALL) rust.tar.gz; \
	fi

centos: prereqs
	cat Dockerfile.centos > Dockerfile && \
	echo >> Dockerfile && \
	cat Dockerfile.common >> Dockerfile && \
	docker build --tag=ned-centos .
	rm Dockerfile
	docker run -it -v /home/nev/dev/ned/build/bin:/root/bin ned-centos
	cp bin/ned bin/ned.centos

ubuntu: prereqs
	cat Dockerfile.ubuntu > Dockerfile && \
	echo >> Dockerfile && \
	cat Dockerfile.common >> Dockerfile && \
	docker build --tag=ned-ubuntu .
	rm Dockerfile
	docker run -it -v /home/nev/dev/ned/build/bin:/root/bin ned-ubuntu
	cp bin/ned bin/ned.ubuntu

deploy: centos ubuntu
	cp ~/dev/ned/build/bin/ned.ubuntu /usr/local/bin/ned
	scp ~/dev/ned/build/bin/ned.ubuntu nevd@nevpc:/usr/local/bin/ned
	scp -P 77 ~/dev/ned/build/bin/ned.centos nevd@porky.benon.com:/home/nevd/ned
