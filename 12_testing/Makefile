# 12_testing Makefile
#
# Targets:
#   check       — compile and run ALL concept files, exercises, and the project
#   concept     — compile and run concept files
#   exercises   — compile and run standalone exercise test files
#   project     — run cargo test and a quick smoke test on the calc project
#   clean       — remove all compiled binaries

CC = rustc
EDITION = --edition 2021
TEST_FLAG = --test

CONCEPT_FILES = \
	concept/01_test_basics.rs \
	concept/02_test_organization.rs \
	concept/03_test_fixtures.rs \
	concept/04_result_in_tests.rs \
	concept/05_doc_tests.rs \
	concept/06_benchmarking.rs \
	concept/07_mocking.rs

EXERCISE_FILES = \
	exercises/ex01_math_test.rs \
	exercises/ex02_string_test.rs \
	exercises/ex03_sort_test.rs

# Build binary names (replace .rs with nothing, strip path)
CONCEPT_BINS = $(CONCEPT_FILES:concept/%.rs=%)
EXERCISE_BINS = $(EXERCISE_FILES:exercises/%.rs=%)

.PHONY: all check concept exercises project clean

all: check

check: concept exercises project
	@echo ""
	@echo "============================================"
	@echo "  ALL CHECKS PASSED"
	@echo "============================================"

# --- Concept files ---

concept: $(CONCEPT_BINS)
	@echo "--- All concept tests passed ---"

$(CONCEPT_BINS): %: concept/%.rs
	$(CC) $(EDITION) $(TEST_FLAG) concept/$@.rs -o concept/$@
	./concept/$@
	@echo "  ✓ concept/$@.rs"

# --- Exercises (standalone) ---

exercises: $(EXERCISE_BINS)
	@echo "--- All exercises passed ---"

$(EXERCISE_BINS): %: exercises/%.rs
	$(CC) $(EDITION) $(TEST_FLAG) exercises/$@.rs -o exercises/$@
	./exercises/$@
	@echo "  ✓ exercises/$@.rs"

# --- Project ---

project:
	@echo "--- Running calc project tests ---"
	cd project/calc && cargo test && cargo build
	@echo "  ✓ calc project: tests + build"
	# Quick smoke test of the CLI
	cd project/calc && \
		[ "$$(cargo run --quiet -- add 2 3)" = "5" ] && \
		[ "$$(cargo run --quiet -- sub 10 4)" = "6" ] && \
		[ "$$(cargo run --quiet -- mul 3 7)" = "21" ] && \
		[ "$$(cargo run --quiet -- div 10 2)" = "5" ] && \
		echo "  ✓ calc CLI smoke test passed"
	# Test error cases
	cd project/calc && \
		result=$$(cargo run --quiet -- div 10 0 2>&1 || true) && \
		echo "$$result" | grep -q "Error: division" && \
		echo "  ✓ calc CLI error handling passed"
	@echo "--- calc project complete ---"

# --- Integration test bonus (separate cargo project) ---

.PHONY: bonus
bonus:
	@echo "--- Running bonus integration test ---"
	cd exercises/ex04_integration_test && cargo test
	@echo "  ✓ bonus integration test crate"

# --- Clean ---

clean:
	rm -f concept/01_test_basics concept/02_test_organization concept/03_test_fixtures
	rm -f concept/04_result_in_tests concept/05_doc_tests concept/06_benchmarking concept/07_mocking
	rm -f exercises/ex01_math_test exercises/ex02_string_test exercises/ex03_sort_test
	rm -rf project/calc/target
	rm -rf exercises/ex04_integration_test/target
	@echo "Cleaned up all compiled binaries and build artifacts"
