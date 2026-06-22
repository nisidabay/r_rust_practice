# 05_references exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/05_references_ex

EXES = $(BIN_DIR)/ex01_swap \
       $(BIN_DIR)/ex02_capitalize \
       $(BIN_DIR)/ex03_find_longest \
       $(BIN_DIR)/ex04_bonus_scoped

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_swap: ex01_swap.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_capitalize: ex02_capitalize.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_find_longest: ex03_find_longest.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_scoped: ex04_bonus_scoped.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_swap === " && $(BIN_DIR)/ex01_swap
	@echo "=== ex02_capitalize === " && $(BIN_DIR)/ex02_capitalize
	@echo "=== ex03_find_longest === " && $(BIN_DIR)/ex03_find_longest
	@echo "=== ex04_bonus_scoped === " && $(BIN_DIR)/ex04_bonus_scoped

clean:
	rm -f $(EXES)
