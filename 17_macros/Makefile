# Makefile for 17_macros group
# Runs all concept files, exercises, and project

RUSTC = rustc --edition 2021
CONCEPT_FILES = $(wildcard concept/0*.rs)
EXERCISE_FILES = $(wildcard exercises/ex0*.rs)

.PHONY: all check clean concept_check exercise_check project_check

all: concept_check exercise_check project_check

check: all

concept_check:
	@echo "=== Checking Concept Files ==="
	@for f in $(CONCEPT_FILES); do \
		echo "  Compiling $$f..."; \
		$(RUSTC) "$$f" 2>&1 && \
		echo "    ✓ $$(basename $$f .rs) compiled successfully"; \
	done
	@echo "  All concept files OK!"
	@echo ""

exercise_check:
	@echo "=== Checking Exercise Files ==="
	@for f in $(EXERCISE_FILES); do \
		echo "  Compiling $$f..."; \
		$(RUSTC) "$$f" 2>&1 && \
		echo "    ✓ $$(basename $$f .rs) compiled successfully"; \
	done
	@echo "  All exercise files OK!"
	@echo ""

project_check:
	@echo "=== Checking Project ==="
	cd project && cargo build 2>&1
	@echo "  ✓ project/macro_madness built successfully"
	cd project && cargo run -- --dump 2>&1
	@echo "  ✓ project/macro_madness --dump ran successfully"
	@echo ""

clean:
	@echo "=== Cleaning ==="
	rm -f concept/0* exercise/ex0* project/target -r
	@echo "  Done!"
