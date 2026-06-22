# 10_errors exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/10_errors_ex

EXES = $(BIN_DIR)/ex01_parse_csv \
       $(BIN_DIR)/ex02_divide \
       $(BIN_DIR)/ex03_config_reader \
       $(BIN_DIR)/ex04_bonus_validate

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_parse_csv: ex01_parse_csv.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_divide: ex02_divide.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_config_reader: ex03_config_reader.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_validate: ex04_bonus_validate.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_parse_csv ===" && printf "Alice,30,NYC\nBob,25,LA\nbadline\nCharlie,35,Chicago\n" | $(BIN_DIR)/ex01_parse_csv
	@echo ""
	@echo "=== ex02_divide ===" && printf "10/3\n5/0\nabc/2\n8/4\n" | $(BIN_DIR)/ex02_divide
	@echo ""
	@echo "=== ex03_config_reader ===" && printf "name=Alice\nage=30\n# comment\ncity=NYC\nname=Bob\nbadline\n" | $(BIN_DIR)/ex03_config_reader
	@echo ""
	@echo "=== ex04_bonus_validate ===" && printf "50\nabc\n0\n150\n100\n" | $(BIN_DIR)/ex04_bonus_validate

clean:
	rm -f $(EXES)
