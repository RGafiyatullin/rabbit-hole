
CARGO = cargo

.PHONY: all
all:

.PHONY: fmt
fmt:
	$(CARGO) +nightly fmt

fmt-check:
	$(CARGO) +nightly fmt --check

.PHONY: test
test:
	$(CARGO) nextest run --release


.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: clippy
clippy:
	$(CARGO) clippy --release
