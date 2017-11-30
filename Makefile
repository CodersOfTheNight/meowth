CARGO_BIN=~/.cargo/bin/cargo
RUSTUP_BIN=~/.cargo/bin/rustup
OPENSSL_PATH="/usr/local/opt/openssl/"
ZMQ_STATE=.zmq

__PHONY__: all

all: build

$(CARGO_BIN):
	curl https://sh.rustup.rs -sSf | sh

build: $(CARGO_BIN) zmq
	$(CARGO_BIN) build

run: $(CARGO_BIN) build
	RUST_LOG=debug $(CARGO_BIN) run -- -c config.yaml

release: $(CARGO_BIN) zmq test
	$(CARGO_BIN) build --release

test: $(CARGO_BIN) build
	$(CARGO_BIN) test

install: $(CARGO_BIN) test release
ifneq ("$(wildcard /etc/redhat-release)","")
	mkdir -p /opt/meowth/
	cp target/release/meowth
	cp etc/meowth.service /etc/systemd/system/meowth.service
	systemctl daemon-reload
else
	@echo "Sorry, but simplified installiation is not available for your OS"
endif

zmq:
ifneq ("$(wildcard $(ZMQ_STATE))","")
	@echo "Zmq is already here"
else ifneq ("$(wildcard /etc/redhat-release)","")
	yum install zeromq-4.1.2 zeromq-devel-4.1.2
else
	sh build_zmq.sh
	touch $(ZMQ_STATE)
endif
