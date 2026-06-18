# Exercise Makefile for 03_ownership
# Run `make check` to compile and run all exercises

STUBS  = ex01_word_counter ex02_string_reverse ex03_split_first
BONUS  = ex04_ownership_debug
SOLVED = ex01_word_counter_solved ex02_string_reverse_solved \
         ex03_split_first_solved ex04_ownership_debug_solved

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
	@if rustc --edition 2021 ex04_ownership_debug.rs -o /tmp/ex04_ownership_debug 2>/dev/null; then \
		echo "ERROR: ex04_ownership_debug should NOT compile!"; \
		exit 1; \
	else \
		echo "  ✓ Correctly fails with ownership errors — fix them!"; \
	fi

# Run all solved exercises — must compile AND pass
check_solved: $(SOLVED)
	@echo "All solved exercises pass!"

$(SOLVED): %: %.rs
	@echo "--- Running $@ ---"
	rustc --edition 2021 $< -o /tmp/$@
	/tmp/$@
	@echo "PASS: $@"

clean:
	rm -f /tmp/ex01_word_counter /tmp/ex01_word_counter_solved
	rm -f /tmp/ex02_string_reverse /tmp/ex02_string_reverse_solved
	rm -f /tmp/ex03_split_first /tmp/ex03_split_first_solved
	rm -f /tmp/ex04_ownership_debug /tmp/ex04_ownership_debug_solved
