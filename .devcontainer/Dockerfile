FROM rust:1-buster

COPY Makefile /tmp/Makefile

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y curl \
	python3 python3-pip nodejs npm make gcc

RUN make -f /tmp/Makefile dependencies
