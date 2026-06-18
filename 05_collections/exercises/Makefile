# 05_collections / exercises — Makefile
#
# Run `make check` to compile and run all exercises.

SRCS := $(wildcard *.rs)
BINS := $(SRCS:.rs=)

.PHONY: all check clean

all: $(BINS)

check: $(BINS)
	@echo "--- Running all exercises ---"
	@for bin in $(BINS); do \
		echo "[$$bin]"; \
		./$$bin; \
		echo "---"; \
	done

%: %.rs
	rustc --edition 2021 $< -o $@

clean:
	rm -f $(BINS)
