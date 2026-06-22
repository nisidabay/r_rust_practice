# 03_control exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/03_control_ex

EXES = $(BIN_DIR)/ex01_fizzbuzz \
       $(BIN_DIR)/ex02_guessing \
       $(BIN_DIR)/ex03_multiplication_table \
       $(BIN_DIR)/ex04_bonus_pi_series

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_fizzbuzz: ex01_fizzbuzz.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_guessing: ex02_guessing.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_multiplication_table: ex03_multiplication_table.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_pi_series: ex04_bonus_pi_series.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_fizzbuzz (first 20 lines) ===" && $(BIN_DIR)/ex01_fizzbuzz | head -20
	@echo "=== ex02_guessing (guess 50,30,40,42) ===" && printf "50\n30\n40\n42\n" | $(BIN_DIR)/ex02_guessing
	@echo "=== ex03_multiplication_table (N=5) ===" && echo "5" | $(BIN_DIR)/ex03_multiplication_table
	@echo "=== ex04_bonus_pi_series ===" && $(BIN_DIR)/ex04_bonus_pi_series

clean:
	rm -f $(EXES)
