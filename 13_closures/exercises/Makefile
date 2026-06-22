# 13_closures exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/13_closures_ex

EXES = $(BIN_DIR)/ex01_filter_map \
       $(BIN_DIR)/ex02_csv_stats \
       $(BIN_DIR)/ex03_word_pipeline \
       $(BIN_DIR)/ex04_bonus_custom_iterator

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_filter_map: ex01_filter_map.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_csv_stats: ex02_csv_stats.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_word_pipeline: ex03_word_pipeline.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_custom_iterator: ex04_bonus_custom_iterator.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_filter_map (input: 1 2 3 4 5) ===" && printf "1\n2\n3\n4\n5\n" | $(BIN_DIR)/ex01_filter_map
	@echo "=== ex02_csv_stats ===" && printf "name,score,age\nAlice,85,30\nBob,92,25\nCharlie,78,35\n" | $(BIN_DIR)/ex02_csv_stats
	@echo "=== ex03_word_pipeline ===" && echo "Hello world! This is a test. Hello again, world." | $(BIN_DIR)/ex03_word_pipeline
	@echo "=== ex04_bonus_custom_iterator ===" && $(BIN_DIR)/ex04_bonus_custom_iterator

clean:
	rm -f $(EXES)
