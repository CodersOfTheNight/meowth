CARGO_BIN=~/.cargo/bin/cargo
OPENSSL_PATH="/usr/local/opt/openssl/"

__PHONY__: all

all: $(CARGO_BIN)
	$(CARGO_BIN) build

$(CARGO_BIN):
	curl https://sh.rustup.rs -sSf | sh

run: $(CARGO_BIN)
	RUST_LOG=debug $(CARGO_BIN) run -- -c config.yaml

release: $(CARGO_BIN) test
	$(CARGO_BIN) build --release

test: $(CARGO_BIN)
	$(CARGO_BIN) test
