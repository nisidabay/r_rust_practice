# Makefile for 15_concurrency exercises
#
# The one question: "How do you run code in parallel — safely?"

.POSIX:

EXERCISES = ex01_parallel_sum ex02_producer_consumer ex03_shared_counter
BONUS     = ex04_parallel_map
RUSTC    ?= rustc
EDITION   = 2021
BIN_DIR   = /tmp

.PHONY: all check clean

all: check

check: $(EXERCISES:%=%) $(BONUS:%=%)
	@echo ""
	@echo "All exercises pass!"

$(BIN_DIR)/ex01_parallel_sum: ex01_parallel_sum.rs
	$(RUSTC) --edition $(EDITION) $< -o $@
	$@

$(BIN_DIR)/ex02_producer_consumer: ex02_producer_consumer.rs
	$(RUSTC) --edition $(EDITION) $< -o $@
	$@

$(BIN_DIR)/ex03_shared_counter: ex03_shared_counter.rs
	$(RUSTC) --edition $(EDITION) $< -o $@
	$@

$(BIN_DIR)/ex04_parallel_map: ex04_parallel_map.rs
	$(RUSTC) --edition $(EDITION) $< -o $@
	$@

ex01_parallel_sum: $(BIN_DIR)/ex01_parallel_sum
	@echo "=== ex01_parallel_sum PASSED ==="

ex02_producer_consumer: $(BIN_DIR)/ex02_producer_consumer
	@echo "=== ex02_producer_consumer PASSED ==="

ex03_shared_counter: $(BIN_DIR)/ex03_shared_counter
	@echo "=== ex03_shared_counter PASSED ==="

ex04_parallel_map: $(BIN_DIR)/ex04_parallel_map
	@echo "=== ex04_parallel_map PASSED ==="

clean:
	rm -f $(BIN_DIR)/ex01_parallel_sum $(BIN_DIR)/ex02_producer_consumer
	rm -f $(BIN_DIR)/ex03_shared_counter $(BIN_DIR)/ex04_parallel_map
	@echo "Cleaned up."
