# Nuke built-in rules and variables.
override MAKEFLAGS += -rR

# Default target.
.PHONY: all
all:
	cargo build --target x86_64-unknown-none
	cp target/x86_64-unknown-none/debug/runix runix.elf

# Remove object files and the final executable.
.PHONY: clean
clean:
	cargo clean
	rm -rf runix.elf

.PHONY: distclean
distclean: clean
