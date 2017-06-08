CARGO_BIN=~/.cargo/bin/cargo
RUSTUP_BIN=~/.cargo/bin/rustup
OPENSSL_PATH="/usr/local/opt/openssl/"

__PHONY__: all

all: build

$(CARGO_BIN):
	curl https://sh.rustup.rs -sSf | sh

build: $(CARGO_BIN) zmq devel_version
	$(CARGO_BIN) build

devel_version:
	$(RUSTUP_BIN) install nightly
	$(RUSTUP_BIN) default nightly


run: $(CARGO_BIN) build
	RUST_LOG=debug $(CARGO_BIN) run -- -c config.yaml

release: $(CARGO_BIN) zmq devel_version test
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
ifneq ("$(wildcard /etc/redhat-release)","")
	yum install zeromq zeromq-devel
endif
