# Makefile for 13_io_filesystem exercises
# Usage: make check   — compiles all exercises
#        make run     — runs all exercises with test data
#        make clean   — removes binaries

EXERCISES = ex01_head ex02_config_reader ex03_file_tree ex04_diff ex05_tail_f
SRCS = $(addsuffix .rs, $(EXERCISES))
BINS = $(EXERCISES)
RUSTC = rustc
RUSTFLAGS = --edition 2021

.PHONY: all check run clean

all: check

check: $(BINS)
	@echo "All exercises compiled successfully."

$(BINS): %: %.rs
	$(RUSTC) $(RUSTFLAGS) -o $@ $<

# Test data
test_config.txt:
	@echo "# Database config" > $@
	@echo "host = localhost" >> $@
	@echo "port = 5432" >> $@
	@echo "db_name = myapp" >> $@
	@echo "debug = true" >> $@

test_a.txt:
	@echo "apple" > $@
	@echo "banana" >> $@
	@echo "cherry" >> $@

test_b.txt:
	@echo "apple" > $@
	@echo "blueberry" >> $@
	@echo "cherry" >> $@
	@echo "date" >> $@

test_multi.txt:
	@for i in $$(seq 1 20); do echo "Line $$i"; done > $@

run: $(BINS) test_config.txt test_a.txt test_b.txt test_multi.txt
	@echo "=== ex01_head ==="
	./ex01_head test_multi.txt 3
	@echo ""
	@echo "=== ex02_config_reader ==="
	./ex02_config_reader test_config.txt
	@echo ""
	@echo "=== ex03_file_tree ==="
	./ex03_file_tree .
	@echo ""
	@echo "=== ex04_diff ==="
	./ex04_diff test_a.txt test_b.txt
	@echo ""
	@echo "=== ex05_tail_f (quick test) ==="
	@echo "Run manually: echo 'new line' >> test_a.txt & ./ex05_tail_f test_a.txt"

clean:
	rm -f $(BINS) test_config.txt test_a.txt test_b.txt test_multi.txt
