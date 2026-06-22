# 01_hello exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/01_hello_ex

EXES = $(BIN_DIR)/ex01_fahrenheit \
       $(BIN_DIR)/ex02_line_stats \
       $(BIN_DIR)/ex03_sum_numbers \
       $(BIN_DIR)/ex04_bonus_collatz

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_fahrenheit: ex01_fahrenheit.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_line_stats: ex02_line_stats.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_sum_numbers: ex03_sum_numbers.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_collatz: ex04_bonus_collatz.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_fahrenheit (25°C) ===" && echo "25" | $(BIN_DIR)/ex01_fahrenheit
	@echo "=== ex02_line_stats ===" && printf "hello world\nrust is fun\n" | $(BIN_DIR)/ex02_line_stats
	@echo "=== ex03_sum_numbers ===" && printf "10\n20\n30\n" | $(BIN_DIR)/ex03_sum_numbers
	@echo "=== ex04_bonus_collatz (13) ===" && echo "13" | $(BIN_DIR)/ex04_bonus_collatz

clean:
	rm -f $(EXES)
