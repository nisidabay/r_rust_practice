# Concept files: compile with rustc (no external deps)
CONCEPTS := 01_async_basics 02_block_on 03_futures_intro \
            04_async_chaining 05_join_concurrent 06_async_vs_thread

# Exercise files
EXERCISES := ex01_simulate_async ex02_fetch_all ex03_timeout ex04_retry

# Targets
.PHONY: all concept exercises project check clean

all: concept exercises project

# --- Concept files ---
concept: $(CONCEPTS:%=concept/%)

concept/%: concept/%.rs
	rustc --edition 2021 $< -o $@
	@echo "  RUN    $@"
	@./$@
	@echo

# --- Exercise files ---
exercises: $(EXERCISES:%=%)

%: %.rs
	rustc --edition 2021 $< -o $@
	@echo "  RUN    $@"
	@./$@
	@echo

# --- Project ---
project:
	cd project && cargo build --release 2>&1
	cd project && cargo run --release -- --help 2>&1

# --- Check target (verify everything compiles and runs) ---
check: clean
	@echo "=== Checking concept files ==="
	@set -e; for f in $(CONCEPTS); do \
		echo "  Compile concept/$$f.rs"; \
		rustc --edition 2021 concept/$$f.rs -o concept/$$f; \
		echo "  Run    concept/$$f"; \
		concept/$$f; \
	done
	@echo
	@echo "=== Checking exercise files ==="
	@set -e; for f in $(EXERCISES); do \
		echo "  Compile $$f.rs"; \
		rustc --edition 2021 $$f.rs -o $$f; \
		echo "  Run    $$f"; \
		./$$f; \
	done
	@echo
	@echo "=== Checking project ==="
	cd project && cargo check 2>&1
	cd project && cargo run --release -- --help 2>&1
	@echo
	@echo "=== All checks passed ==="

clean:
	rm -f $(CONCEPTS:%=concept/%) $(EXERCISES:%./%)
	rm -rf project/target
	@echo "Cleaned build artifacts"
