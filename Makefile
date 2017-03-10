CARGO_BIN=~/.cargo/bin/cargo
OPENSSL_PATH="/usr/local/opt/openssl/"

__PHONY__:
	$(CARGO_BIN) build

run:
	$(CARGO_BIN) run -- -c config.yaml
linux:
	OPENSSL_DIR=$(OPENSSL_PATH) PKG_CONFIG_ALLOW_CROSS=1 $(CARGO_BIN) build --target=x86_64-unknown-linux-gnu
