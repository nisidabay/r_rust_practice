# 07_error_handling / exercises — Makefile
#
# Run `make check` to compile and run all exercises.
# Use `make run_ex04` to run the bonus exercise by itself.

SRCS := $(filter-out ex04_validate_csv.rs, $(wildcard *.rs))
BINS := $(SRCS:.rs=)
BONUS := ex04_validate_csv

.PHONY: all check clean run_ex04

all: $(BINS) $(BONUS)

check: $(BINS)
	@echo "--- Running standard exercises ---"
	@for bin in $(BINS); do \
		echo "[$$bin]"; \
		./$$bin; \
		echo "---"; \
	done
	@echo ""
	@echo "=== Bonus exercise (ex04) is available — run 'make run_ex04' ==="

run_ex04: $(BONUS)
	@echo "[$(BONUS)]"
	@./$(BONUS)

%: %.rs
	rustc --edition 2021 $< -o $@

clean:
	rm -f $(BINS) $(BONUS)
