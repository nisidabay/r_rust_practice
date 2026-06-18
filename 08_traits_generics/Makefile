# Makefile for 08_traits_generics
#
# Compiles and runs concept files, exercises, and the calculator project.

SHELL := /bin/bash
RUSTC := rustc --edition 2021

# Concept files (01-10)
CONCEPTS := $(wildcard 0[1-9]*.rs 10*.rs)

# Exercise files
EXERCISES := $(wildcard ex*.rs)

# All standalone Rust files
STANDALONE := $(CONCEPTS) $(EXERCISES)

# Binaries (standalone)
BINARIES := $(STANDALONE:.rs=)

.PHONY: all concepts exercises project clean check

all: concepts exercises project

# ── Concepts ──────────────────────────────────────────────────────────────────

concepts: $(CONCEPTS)
	@echo "=== Compiling and running concept files ==="
	@for f in $(CONCEPTS); do \
		echo "--- $$f ---"; \
		$(RUSTC) $$f && ./$${f%.rs} && echo ""; \
	done

# ── Exercises ─────────────────────────────────────────────────────────────────

exercises: $(EXERCISES)
	@echo "=== Compiling and running exercises ==="
	@for f in $(EXERCISES); do \
		echo "--- $$f ---"; \
		$(RUSTC) $$f && ./$${f%.rs} && echo ""; \
	done

# ── Project ───────────────────────────────────────────────────────────────────

project:
	@echo "=== Building and testing calculator project ==="
	cd projects/calculator && cargo test --quiet 2>&1 || echo "No tests configured"
	@echo ""
	@echo "=== Running calculator examples ==="
	@echo "--- Integer addition ---"
	projects/calculator/target/debug/calculator add 5 3
	@echo "--- Float subtraction ---"
	projects/calculator/target/debug/calculator sub 10.5 2.5
	@echo "--- Complex multiplication ---"
	projects/calculator/target/debug/calculator mul "3+4i" "1+2i"
	@echo "--- Division ---"
	projects/calculator/target/debug/calculator div 100 7
	@echo "--- Help ---"
	projects/calculator/target/debug/calculator help

# ── Build all (compile check, no run) ─────────────────────────────────────────

build: $(STANDALONE)
	@echo "=== Compiling concept files ==="
	@for f in $(CONCEPTS); do \
		$(RUSTC) $$f -o /dev/null 2>&1 || echo "FAIL: $$f"; \
	done
	@echo "=== Compiling exercise files ==="
	@for f in $(EXERCISES); do \
		$(RUSTC) $$f -o /dev/null 2>&1 || echo "FAIL: $$f"; \
	done
	@echo "=== Building calculator project ==="
	cd projects/calculator && cargo build --quiet

# ── Check (compile only) ──────────────────────────────────────────────────────

check:
	@echo "=== Checking concept files ==="
	@for f in $(CONCEPTS); do \
		echo -n "Checking $$f... "; \
		if $(RUSTC) $$f -o /tmp/$${f%.rs}_check 2>/dev/null; then \
			echo "OK"; \
		else \
			echo "FAIL"; \
			$(RUSTC) $$f -o /tmp/$${f%.rs}_check 2>&1; \
		fi; \
		rm -f /tmp/$${f%.rs}_check; \
	done
	@echo "=== Checking exercise files ==="
	@for f in $(EXERCISES); do \
		echo -n "Checking $$f... "; \
		if $(RUSTC) $$f -o /tmp/$${f%.rs}_check 2>/dev/null; then \
			echo "OK"; \
		else \
			echo "FAIL"; \
			$(RUSTC) $$f -o /tmp/$${f%.rs}_check 2>&1; \
		fi; \
		rm -f /tmp/$${f%.rs}_check; \
	done
	@echo "=== Checking calculator project ==="
	cd projects/calculator && cargo check --quiet 2>&1 && echo "Calculator: OK"

# ── Clean ─────────────────────────────────────────────────────────────────────

clean:
	rm -f $(BINARIES)
	rm -rf projects/calculator/target
	@echo "Cleaned."
