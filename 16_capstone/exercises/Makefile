CC = rustc
CFLAGS = --edition 2021
BIN_DIR = /tmp/16_capstone_ex

EXES = $(BIN_DIR)/ex01_grep \
       $(BIN_DIR)/ex02_cat \
       $(BIN_DIR)/ex03_tee \
       $(BIN_DIR)/ex04_bonus_cut

.PHONY: all clean run

all: $(BIN_DIR) $(EXES)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BIN_DIR)/ex01_grep: ex01_grep.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex02_cat: ex02_cat.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex03_tee: ex03_tee.rs
	$(CC) $(CFLAGS) -o $@ $<

$(BIN_DIR)/ex04_bonus_cut: ex04_bonus_cut.rs
	$(CC) $(CFLAGS) -o $@ $<

run: all
	@echo "=== ex01_grep ===" && printf "hello world\nrust is fun\n" | $(BIN_DIR)/ex01_grep rust
	@echo "=== ex02_cat (-n) ===" && echo "hello" | $(BIN_DIR)/ex02_cat -n
	@echo "=== ex03_tee ===" && echo "test line" | $(BIN_DIR)/ex03_tee /tmp/tee_test_out
	@echo "=== ex04_bonus_cut ===" && echo "a,b,c" | $(BIN_DIR)/ex04_bonus_cut -d, -f1,3

clean:
	rm -f $(EXES)
