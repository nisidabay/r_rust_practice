# Exercise Makefile for 10_closures_iterators
# Targets: all (default), check (compile + run), clean

EXERCISES := ex01_filter_map ex02_csv_stats ex03_word_count_pipeline ex04_custom_iterator
SRCS      := $(addsuffix .rs, $(EXERCISES))
BINS      := $(addprefix bin/, $(EXERCISES))
EDITION   := 2021

.PHONY: all check clean

all: $(BINS)

# Pattern rule: compile each .rs into bin/
bin/%: %.rs
	@mkdir -p bin
	rustc --edition $(EDITION) $< -o $@

# Check target: compile, then run each exercise with default args
check: $(BINS)
	@echo "=== ex01_filter_map (default) ==="
	./bin/ex01_filter_map
	@echo ""
	@echo "=== ex02_csv_stats (default) ==="
	./bin/ex02_csv_stats
	@echo ""
	@echo "=== ex03_word_count_pipeline (default) ==="
	./bin/ex03_word_count_pipeline
	@echo ""
	@echo "=== ex04_custom_iterator (default) ==="
	./bin/ex04_custom_iterator
	@echo ""
	@echo "=== ex04_custom_iterator (--filter-even) ==="
	./bin/ex04_custom_iterator --filter-even --count 8
	@echo ""
	@echo "=== All exercises passed ==="

clean:
	rm -rf bin
