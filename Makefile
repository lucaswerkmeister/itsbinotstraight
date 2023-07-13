.PHONY: check

CARGO = cargo

check:
	$(CARGO) $(CARGOFLAGS) check
	$(CARGO) $(CARGOFLAGS) test
	$(CARGO) $(CARGOFLAGS) clippy
