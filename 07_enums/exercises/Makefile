# 07_enums exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/07_enums_ex

EXES = $(BIN_DIR)/ex01_http_status \
       $(BIN_DIR)/ex02_color_hex \
       $(BIN_DIR)/ex03_calculator \
       $(BIN_DIR)/ex04_bonus_command

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_http_status: ex01_http_status.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_color_hex: ex02_color_hex.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_calculator: ex03_calculator.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_command: ex04_bonus_command.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_http_status ===" && $(BIN_DIR)/ex01_http_status
	@echo "=== ex02_color_hex ===" && $(BIN_DIR)/ex02_color_hex
	@echo "=== ex03_calculator ===" && printf "3 + 4\n10 / 2\n8 x 3\n" | $(BIN_DIR)/ex03_calculator
	@echo "=== ex04_bonus_command ===" && printf "add hello\nadd world\nlist\nquit\n" | $(BIN_DIR)/ex04_bonus_command

clean:
	rm -f $(EXES)
