# Makefile for 16_unsafe_ffi
#
# Targets:
#   check       — compile and run all concept files
#   check-ex    — compile and run all exercises
#   check-proj  — compile and run the hex_dump project
#   all         — everything above
#   clean       — remove build artifacts

RUSTC := rustc --edition 2021
CONCEPT_DIR := concept
EXERCISES_DIR := exercises
PROJECT_DIR := project/hex_dump

CONCEPT_FILES := \
	$(CONCEPT_DIR)/01_unsafe_intro.rs \
	$(CONCEPT_DIR)/02_raw_pointers.rs \
	$(CONCEPT_DIR)/03_unsafe_fns.rs \
	$(CONCEPT_DIR)/04_union_ffi.rs \
	$(CONCEPT_DIR)/05_mut_static.rs \
	$(CONCEPT_DIR)/06_unsafe_trait.rs \
	$(CONCEPT_DIR)/07_transmute.rs \
	$(CONCEPT_DIR)/08_abi_calling.rs

.PHONY: all check check-ex check-proj clean

all: check check-ex check-proj

# --- Concept files ---
check: $(CONCEPT_FILES)
	@echo "=== Compiling and running concept files ==="
	@set -e; for f in $(CONCEPT_FILES); do \
		echo "--- $$f ---"; \
		$(RUSTC) "$$f" 2>&1 && ./"$$(basename "$$f" .rs)" && echo ""; \
	done
	@echo "All concept files passed!"

# --- Exercises ---
check-ex:
	@echo "=== Building and running exercises ==="
	@set -e; for dir in ex01_split_buffer ex02_c_string ex03_callback bonus_ex04_lock_api; do \
		echo "--- exercises/$$dir ---"; \
		cd "$(EXERCISES_DIR)/$$dir" && cargo run --quiet 2>&1 && echo "" && cd ../..; \
	done
	@echo "All exercises passed!"

# --- Project ---
check-proj:
	@echo "=== Building hex_dump project ==="
	cd "$(PROJECT_DIR)" && cargo build --quiet 2>&1
	@echo "hex_dump build successful!"
	# Test with a small binary file — dump its own source
	@echo "--- hex_dump --help ---"
	cd "$(PROJECT_DIR)" && cargo run --quiet -- --help 2>&1
	@echo ""
	@echo "--- hex_dump Cargo.toml ---"
	cd "$(PROJECT_DIR)" && cargo run --quiet -- Cargo.toml 2>&1
	@echo ""
	@echo "Project check passed!"

# --- Clean ---
clean:
	@echo "Cleaning build artifacts..."
	rm -f $(CONCEPT_DIR)/01_unsafe_intro
	rm -f $(CONCEPT_DIR)/02_raw_pointers
	rm -f $(CONCEPT_DIR)/03_unsafe_fns
	rm -f $(CONCEPT_DIR)/04_union_ffi
	rm -f $(CONCEPT_DIR)/05_mut_static
	rm -f $(CONCEPT_DIR)/06_unsafe_trait
	rm -f $(CONCEPT_DIR)/07_transmute
	rm -f $(CONCEPT_DIR)/08_abi_calling
	cd "$(PROJECT_DIR)" && cargo clean 2>/dev/null || true
	@for dir in ex01_split_buffer ex02_c_string ex03_callback bonus_ex04_lock_api; do \
		cd "$(EXERCISES_DIR)/$$dir" && cargo clean 2>/dev/null || true; cd ../..; \
	done
	@echo "Clean complete!"
