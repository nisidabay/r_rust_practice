CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/15_testing_ex

EXES = $(BIN_DIR)/ex01_math \
       $(BIN_DIR)/ex02_string \
       $(BIN_DIR)/ex03_median \
       $(BIN_DIR)/ex04_bonus_property

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_math: ex01_math.rs
	$(CC) $(CFLAGS) --test -o $@ $<

$(BIN_DIR)/ex02_string: ex02_string.rs
	$(CC) $(CFLAGS) --test -o $@ $<

$(BIN_DIR)/ex03_median: ex03_median.rs
	$(CC) $(CFLAGS) --test -o $@ $<

$(BIN_DIR)/ex04_bonus_property: ex04_bonus_property.rs
	$(CC) $(CFLAGS) --test -o $@ $<

run: all
	@echo "=== ex01_math ===" && $(BIN_DIR)/ex01_math
	@echo "=== ex02_string ===" && $(BIN_DIR)/ex02_string
	@echo "=== ex03_median ===" && $(BIN_DIR)/ex03_median
	@echo "=== ex04_bonus_property ===" && $(BIN_DIR)/ex04_bonus_property

clean:
	rm -f $(EXES)
