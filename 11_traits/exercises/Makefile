# 11_traits exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/11_traits_ex

EXES = $(BIN_DIR)/ex01_area \
       $(BIN_DIR)/ex02_to_base \
       $(BIN_DIR)/ex03_pretty \
       $(BIN_DIR)/ex04_bonus_complex

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_area: ex01_area.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_to_base: ex02_to_base.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_pretty: ex03_pretty.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_complex: ex04_bonus_complex.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_area ===" && $(BIN_DIR)/ex01_area
	@echo ""
	@echo "=== ex02_to_base ===" && echo "255" | $(BIN_DIR)/ex02_to_base
	@echo ""
	@echo "=== ex03_pretty ===" && $(BIN_DIR)/ex03_pretty
	@echo ""
	@echo "=== ex04_bonus_complex ===" && $(BIN_DIR)/ex04_bonus_complex

clean:
	rm -f $(EXES)
