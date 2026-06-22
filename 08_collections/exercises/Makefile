# 08_collections exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/08_collections_ex

EXES = $(BIN_DIR)/ex01_word_freq \
       $(BIN_DIR)/ex02_merge_dedup \
       $(BIN_DIR)/ex03_palindrome \
       $(BIN_DIR)/ex04_bonus_inverted_index

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_word_freq: ex01_word_freq.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_merge_dedup: ex02_merge_dedup.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_palindrome: ex03_palindrome.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_inverted_index: ex04_bonus_inverted_index.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_word_freq ===" && printf "hello world hello\nworld hello\nfoo bar\n" | $(BIN_DIR)/ex01_word_freq
	@echo "=== ex02_merge_dedup ===" && printf "apple, banana, cherry\nbanana, date, apple, fig\n" | $(BIN_DIR)/ex02_merge_dedup
	@echo "=== ex03_palindrome ===" && printf "racecar\nhello\nA man a plan a canal Panama\n" | $(BIN_DIR)/ex03_palindrome
	@echo "=== ex04_bonus_inverted_index ===" && printf "hello world\nfoo bar\nhello foo\n" | $(BIN_DIR)/ex04_bonus_inverted_index

clean:
	rm -f $(EXES)
