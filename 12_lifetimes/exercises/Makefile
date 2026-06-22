# 12_lifetimes exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/12_lifetimes_ex

EXES = $(BIN_DIR)/ex01_longest \
       $(BIN_DIR)/ex02_parser \
       $(BIN_DIR)/ex03_joiner \
       $(BIN_DIR)/ex04_bonus_messenger

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_longest: ex01_longest.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_parser: ex02_parser.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_joiner: ex03_joiner.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_messenger: ex04_bonus_messenger.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_longest ===" && $(BIN_DIR)/ex01_longest
	@echo "=== ex02_parser ===" && $(BIN_DIR)/ex02_parser
	@echo "=== ex03_joiner ===" && $(BIN_DIR)/ex03_joiner
	@echo "=== ex04_bonus_messenger ===" && $(BIN_DIR)/ex04_bonus_messenger

clean:
	rm -f $(EXES)
