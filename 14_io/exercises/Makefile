# 14_io exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/14_io_ex

EXES = $(BIN_DIR)/ex01_head \
       $(BIN_DIR)/ex02_config_reader \
       $(BIN_DIR)/ex03_file_stats \
       $(BIN_DIR)/ex04_bonus_find

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_head: ex01_head.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_config_reader: ex02_config_reader.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_file_stats: ex03_file_stats.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_find: ex04_bonus_find.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_head (10 lines of Makefile) ===" && $(BIN_DIR)/ex01_head Makefile 10
	@echo ""
	@echo "=== ex02_config_reader (stdin config) ===" && printf "name=Alice\nage=30\n# comment\ncity=NYC\n" | $(BIN_DIR)/ex02_config_reader
	@echo ""
	@echo "=== ex03_file_stats (Makefile + README.md) ===" && $(BIN_DIR)/ex03_file_stats Makefile ../README.md
	@echo ""
	@echo "=== ex04_bonus_find (pattern 'rs' in ..) ===" && $(BIN_DIR)/ex04_bonus_find .rs ..

clean:
	rm -f $(EXES)
