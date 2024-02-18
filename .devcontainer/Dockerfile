FROM rust:1-buster

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y curl python3 python3-pip make gcc && \
	curl -fsSL https://deb.nodesource.com/setup_21.x | bash - && \
	apt-get install -y nodejs

RUN rustup component add clippy

COPY Makefile /tmp/Makefile
RUN make -j -f /tmp/Makefile dependencies
