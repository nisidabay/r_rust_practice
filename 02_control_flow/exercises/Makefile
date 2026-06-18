# Exercise Makefile for 02_control_flow
# Targets: check (compile all), run (compile + run all)

EXERCISES = ex01_fizzbuzz ex02_guessing ex03_roman ex04_collatz
SRC_DIR = .
BIN_DIR = /tmp
RUSTC = rustc --edition 2021

.PHONY: all check run clean

all: check

# Compile all exercises; fail fast on first error.
check:
	@for ex in $(EXERCISES); do \
		echo "  Compiling $$ex..."; \
		$(RUSTC) $(SRC_DIR)/$$ex.rs -o $(BIN_DIR)/$$ex && echo "    OK"; \
	done
	@echo "All exercises compiled successfully."

# Compile and run all exercises.
run: check
	@for ex in $(EXERCISES); do \
		echo ""; \
		echo "=== $$ex ==="; \
		case $$ex in \
			ex02_guessing) echo "5" | $(BIN_DIR)/$$ex ;; \
			*) $(BIN_DIR)/$$ex ;; \
		esac; \
	done

clean:
	@for ex in $(EXERCISES); do \
		rm -f $(BIN_DIR)/$$ex; \
	done
	@echo "Cleaned up binaries."
