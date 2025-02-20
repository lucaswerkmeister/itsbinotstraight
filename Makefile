.PHONY: check clean

CARGO = cargo

check:
	$(CARGO) $(CARGOFLAGS) check
	$(CARGO) $(CARGOFLAGS) test
	$(CARGO) $(CARGOFLAGS) clippy

clean:
	$(CARGO) $(CARGOFLAGS) clean
