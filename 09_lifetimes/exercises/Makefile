# Exercise Makefile for 09_lifetimes
# Run `make check` to compile and run all exercises

STUBS  = ex01_longest ex02_parser ex03_ring_buffer
BONUS  = ex04_lifetime_errors
SOLVED = ex01_longest_solved ex02_parser_solved ex03_ring_buffer_solved \
         ex04_lifetime_errors_solved

.PHONY: all check check_stubs check_bonus check_solved clean

all: check

# Default check: compile stubs (expect todo!()), verify solved run, and verify
# the BONUS buggy exercise FAILS at compile time (so students learn the errors)
check: check_stubs check_solved check_bonus
	@echo "============================================"
	@echo "All exercises check out!"
	@echo "============================================"

# Compile stub solutions (they have todo!() — compiles but panics at runtime)
check_stubs: $(STUBS:%=%.rs)
	@echo "--- Compiling stub exercises ---"
	@for f in $(STUBS); do \
		echo "  Compiling $$f.rs..."; \
		rustc --edition 2021 $$f.rs -o /tmp/$$f; \
		echo "  OK (todo!() will panic at runtime — expected)"; \
	done

# Verify BONUS buggy file FAILS to compile (it's intentionally broken)
check_bonus:
	@echo "--- Verifying BONUS exercise fails to compile (intentionally broken) ---"
	@if rustc --edition 2021 ex04_lifetime_errors.rs -o /tmp/ex04_lifetime_errors 2>/dev/null; then \
		echo "ERROR: ex04_lifetime_errors should NOT compile!"; \
		exit 1; \
	else \
		echo "  ✓ Correctly fails with lifetime errors — fix them!"; \
	fi

# Run all solved exercises — must compile AND pass
check_solved: $(SOLVED)
	@echo "All solved exercises pass!"

$(SOLVED): %: %.rs
	@echo "--- Running $@ ---"
	@rustc --edition 2021 $< -o /tmp/$@
	@/tmp/$@
	@echo "PASS: $@"

clean:
	rm -f /tmp/ex01_longest /tmp/ex01_longest_solved
	rm -f /tmp/ex02_parser /tmp/ex02_parser_solved
	rm -f /tmp/ex03_ring_buffer /tmp/ex03_ring_buffer_solved
	rm -f /tmp/ex04_lifetime_errors /tmp/ex04_lifetime_errors_solved
