FROM ubuntu:22.04

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y curl python3 python3-pip make gcc git pkg-config libssl-dev

RUN curl -fsSL https://deb.nodesource.com/setup_21.x | bash - && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y nodejs

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
	echo "source $HOME/.cargo/env" >> $HOME/.bashrc

RUN	$HOME/.cargo/bin/rustup component add clippy

RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown

RUN . $HOME/.cargo/env && \
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY Makefile /tmp/Makefile
RUN . $HOME/.cargo/env && \
	make -f /tmp/Makefile dependencies
