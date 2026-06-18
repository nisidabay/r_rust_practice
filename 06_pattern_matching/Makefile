# Makefile for 06_pattern_matching
# Standard targets: check, clean

CHECK_MARKS = ✓

# Concept files — compile-only (they have main())
CONCEPT_FILES := $(wildcard concept/*.rs)
CONCEPT_BINS := $(CONCEPT_FILES:.rs=)

# Exercise files — compile-only
EXERCISE_FILES := $(wildcard exercises/*.rs)
EXERCISE_BINS := $(EXERCISE_FILES:.rs=)

# Project
PROJECT_DIR := project

.PHONY: all check concept-check exercise-check project-check clean

all: check

check: concept-check exercise-check project-check
	@echo "All checks passed $(CHECK_MARKS)"

# Compile and run each concept file
concept-check:
	@for f in $(CONCEPT_FILES); do \
		bin="$${f%.rs}"; \
		echo "  COMPILE  $$f"; \
		rustc --edition 2021 "$$f" -o "$$bin" 2>&1; \
		echo "  RUN      $$bin"; \
		./"$$bin" > /dev/null 2>&1 && echo "  $(CHECK_MARKS) $$f passed" || (echo "  FAILED $$f"; exit 1); \
	done

# Compile each exercise file (they have todo!() so only compile, don't run)
exercise-check: $(EXERCISE_FILES)
	@for f in $^; do \
		echo "  COMPILE  $$f"; \
		rustc --edition 2021 "$$f" -o "$${f%.rs}" 2>&1 && echo "  $(CHECK_MARKS) $$f compiles" || (echo "  FAILED $$f"; exit 1); \
	done

# Build and test the project
project-check:
	@echo "  BUILD    $(PROJECT_DIR)"
	@cd $(PROJECT_DIR) && cargo build 2>&1
	@echo "  TEST     $(PROJECT_DIR)"
	@cd $(PROJECT_DIR) && cargo test 2>&1
	@echo "  $(CHECK_MARKS) project passed"

clean:
	rm -f concept/*.exe concept/*.dSYM concept/*.pdb concept/0[1-7]_match_*
	rm -f exercises/ex0[1-4]_*.exe exercises/ex0[1-4]_*.dSYM
	-cd $(PROJECT_DIR) && cargo clean 2>/dev/null || true
	@echo "Cleaned"
