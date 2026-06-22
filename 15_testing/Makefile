# 15_testing — Run all tests
#
# Exercises: compile with rustc --test and run
# Project:   cargo test

CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/15_testing_ex

EXES = $(BIN_DIR)/ex01_math \
       $(BIN_DIR)/ex02_string \
       $(BIN_DIR)/ex03_median \
       $(BIN_DIR)/ex04_bonus_property

.PHONY: all clean run project_test

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_math: exercises/ex01_math.rs
	$(CC) --test $(CFLAGS) -o $@ exercises/ex01_math.rs

$(BIN_DIR)/ex02_string: exercises/ex02_string.rs
	$(CC) --test $(CFLAGS) -o $@ exercises/ex02_string.rs

$(BIN_DIR)/ex03_median: exercises/ex03_median.rs
	$(CC) --test $(CFLAGS) -o $@ exercises/ex03_median.rs

$(BIN_DIR)/ex04_bonus_property: exercises/ex04_bonus_property.rs
	$(CC) --test $(CFLAGS) -o $@ exercises/ex04_bonus_property.rs

run: all
	@echo "=== ex01_math ===" && $(BIN_DIR)/ex01_math
	@echo ""
	@echo "=== ex02_string ===" && $(BIN_DIR)/ex02_string
	@echo ""
	@echo "=== ex03_median ===" && $(BIN_DIR)/ex03_median
	@echo ""
	@echo "=== ex04_bonus_property ===" && $(BIN_DIR)/ex04_bonus_property

# Run project tests with cargo test
project_test:
	cd project && cargo test

# Run everything
test: run project_test

clean:
	rm -f $(EXES)
