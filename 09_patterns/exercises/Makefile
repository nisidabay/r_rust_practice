# 09_patterns exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/09_patterns_ex

EXES = $(BIN_DIR)/ex01_parse_log \
       $(BIN_DIR)/ex02_rgb_to_hex \
       $(BIN_DIR)/ex03_arg_parser \
       $(BIN_DIR)/ex04_bonus_nested

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_parse_log: ex01_parse_log.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_rgb_to_hex: ex02_rgb_to_hex.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_arg_parser: ex03_arg_parser.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_nested: ex04_bonus_nested.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_parse_log ===" && printf "ERROR: disk full\nINFO: starting\nWARN: low memory\n" | $(BIN_DIR)/ex01_parse_log
	@echo "=== ex02_rgb_to_hex ===" && printf "#FF8800\nrgb(255,136,0)\n#00FF00\n" | $(BIN_DIR)/ex02_rgb_to_hex
	@echo "=== ex03_arg_parser ===" && $(BIN_DIR)/ex03_arg_parser --name=Alice --count=5 --verbose
	@echo "=== ex04_bonus_nested ===" && printf "(add 1 (mul 2 3))\n(sub 10 3)\n" | $(BIN_DIR)/ex04_bonus_nested

clean:
	rm -f $(EXES)
