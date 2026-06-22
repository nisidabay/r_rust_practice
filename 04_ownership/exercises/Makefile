# 04_ownership exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/04_ownership_ex

EXES = $(BIN_DIR)/ex01_word_counter \
       $(BIN_DIR)/ex02_string_reverse \
       $(BIN_DIR)/ex03_first_word \
       $(BIN_DIR)/ex04_bonus_debug

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_word_counter: ex01_word_counter.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_string_reverse: ex02_string_reverse.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_first_word: ex03_first_word.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_debug: ex04_bonus_debug.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_word_counter === " && echo "hello world foo bar" | $(BIN_DIR)/ex01_word_counter
	@echo "=== ex02_string_reverse === " && echo "hello" | $(BIN_DIR)/ex02_string_reverse
	@echo "=== ex03_first_word === " && echo "hello world" | $(BIN_DIR)/ex03_first_word
	@echo "=== ex04_bonus_debug === " && $(BIN_DIR)/ex04_bonus_debug

clean:
	rm -f $(EXES)
