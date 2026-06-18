# Exercise Makefile for 01_hello_types
# Targets: all (default), check (compile + run), clean

EXERCISES := ex01_temperature ex02_fibonacci ex03_twelve_days ex04_bonus_pi
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
	@echo "=== ex01_temperature (default 0°C) ==="
	./bin/ex01_temperature
	@echo "=== ex01_temperature (100°C) ==="
	./bin/ex01_temperature 100
	@echo "=== ex02_fibonacci (default 10) ==="
	./bin/ex02_fibonacci
	@echo "=== ex02_fibonacci (20) ==="
	./bin/ex02_fibonacci 20
	@echo "=== ex03_twelve_days (full) ==="
	./bin/ex03_twelve_days
	@echo "=== ex04_bonus_pi (1000 terms) ==="
	./bin/ex04_bonus_pi 1000
	@echo "=== All exercises passed ==="

clean:
	rm -rf bin
