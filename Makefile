
CARGO = cargo
CARGO_TEST = cargo nextest run

.PHONY: all
all:

.PHONY: fmt
fmt:
	$(CARGO) +nightly fmt

fmt-check:
	$(CARGO) +nightly fmt --check

.PHONY: test
test:
	$(CARGO_TEST) --release


.PHONY: build
build:
	$(CARGO) build --release --all

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: clippy
clippy:
	$(CARGO) clippy --release
