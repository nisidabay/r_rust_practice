# 02_data exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/02_data_ex

EXES = $(BIN_DIR)/ex01_unit_convert \
       $(BIN_DIR)/ex02_temperature_table \
       $(BIN_DIR)/ex03_compound_interest \
       $(BIN_DIR)/ex04_bonus_primes

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_unit_convert: ex01_unit_convert.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_temperature_table: ex02_temperature_table.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_compound_interest: ex03_compound_interest.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_primes: ex04_bonus_primes.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_unit_convert (100 cm to m) ===" && echo "100 cm to m" | $(BIN_DIR)/ex01_unit_convert
	@echo "=== ex02_temperature_table ===" | $(BIN_DIR)/ex02_temperature_table
	@echo "=== ex03_compound_interest (P=1000, r=5%, n=10) ===" && printf "1000\n5\n10\n" | $(BIN_DIR)/ex03_compound_interest
	@echo "=== ex04_bonus_primes (N=50) ===" && echo "50" | $(BIN_DIR)/ex04_bonus_primes

clean:
	rm -f $(EXES)
