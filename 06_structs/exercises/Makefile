# 06_structs exercises
CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/06_structs_ex

EXES = $(BIN_DIR)/ex01_rectangle \
       $(BIN_DIR)/ex02_book \
       $(BIN_DIR)/ex03_student \
       $(BIN_DIR)/ex04_bonus_bank

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_rectangle: ex01_rectangle.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_book: ex02_book.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_student: ex03_student.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_bank: ex04_bonus_bank.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_rectangle === " && $(BIN_DIR)/ex01_rectangle
	@echo "=== ex02_book === " && $(BIN_DIR)/ex02_book
	@echo "=== ex03_student === " && $(BIN_DIR)/ex03_student
	@echo "=== ex04_bonus_bank === " && $(BIN_DIR)/ex04_bonus_bank

clean:
	rm -f $(EXES)
