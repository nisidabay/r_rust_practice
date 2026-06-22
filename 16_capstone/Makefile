# Group 16 Capstone - Makefile
# Compiles and tests all concept files and exercises

RUSTC = rustc --edition 2021
CONCEPTS = 01_cli_args 02_file_io 03_result_chain 04_struct_app 05_everything_together
EXERCISES = ex01_grep ex02_cat ex03_tee ex04_bonus_cut
ALL_BINS = $(CONCEPTS) $(EXERCISES)

.PHONY: all concepts exercises clean test

all: concepts exercises

# --- Concept files ---

concepts: $(CONCEPTS)

01_cli_args: 01_cli_args.rs
	$(RUSTC) 01_cli_args.rs -o $@

02_file_io: 02_file_io.rs
	$(RUSTC) 02_file_io.rs -o $@

03_result_chain: 03_result_chain.rs
	$(RUSTC) 03_result_chain.rs -o $@

04_struct_app: 04_struct_app.rs
	$(RUSTC) 04_struct_app.rs -o $@

05_everything_together: 05_everything_together.rs
	$(RUSTC) 05_everything_together.rs -o $@

# --- Exercise files ---

exercises: $(EXERCISES)

ex01_grep: ex01_grep.rs
	$(RUSTC) ex01_grep.rs -o $@

ex02_cat: ex02_cat.rs
	$(RUSTC) ex02_cat.rs -o $@

ex03_tee: ex03_tee.rs
	$(RUSTC) ex03_tee.rs -o $@

ex04_bonus_cut: ex04_bonus_cut.rs
	$(RUSTC) ex04_bonus_cut.rs -o $@

# --- Test each binary ---

test: test_concepts test_exercises

test_concepts: concepts
	@echo ""
	@echo "=== Testing 01_cli_args ==="
	./01_cli_args --name=Alice --verbose --count=3
	./01_cli_args --help
	./01_cli_args -h
	@echo ""
	@echo "=== Testing 02_file_io ==="
	./02_file_io
	@echo ""
	@echo "=== Testing 03_result_chain ==="
	./03_result_chain
	@echo ""
	@echo "=== Testing 04_struct_app (non-interactive) ==="
	# Non-interactive: list then quit
	echo -e "list\nquit" | timeout 2 ./04_struct_app || true
	@echo ""
	@echo "=== Testing 05_everything_together ==="
	./05_everything_together list
	./05_everything_together search alice

test_exercises: exercises
	@echo ""
	@echo "=== Testing ex01_grep ==="
	@echo "All the world's a stage" > /tmp/grep_test.txt
	@echo "And all the men and women merely players" >> /tmp/grep_test.txt
	@echo "They have their exits and their entrances" >> /tmp/grep_test.txt
	@echo "and one man in his time plays many parts" >> /tmp/grep_test.txt
	./ex01_grep the < /tmp/grep_test.txt
	@echo "--- case insensitive ---"
	./ex01_grep AND -i < /tmp/grep_test.txt
	@echo "--- with line numbers ---"
	./ex01_grep the -n < /tmp/grep_test.txt
	@echo "--- count only ---"
	./ex01_grep the -c < /tmp/grep_test.txt
	@rm /tmp/grep_test.txt
	@echo ""
	@echo "=== Testing ex02_cat ==="
	@echo "line one" > /tmp/cat_test.txt
	@echo "line two" >> /tmp/cat_test.txt
	@echo "line three" >> /tmp/cat_test.txt
	./ex02_cat /tmp/cat_test.txt
	@echo "--- with line numbers ---"
	./ex02_cat -n /tmp/cat_test.txt
	@echo "--- binary file detection ---"
	python3 -c "import sys; sys.stdout.buffer.write(b'hello\x00world')" > /tmp/bin_test.bin 2>/dev/null || \
	printf 'hello\x00world' > /tmp/bin_test.bin
	./ex02_cat /tmp/bin_test.bin 2>/dev/null || true
	@rm -f /tmp/cat_test.txt /tmp/bin_test.bin
	@echo ""
	@echo "=== Testing ex03_tee ==="
	@echo "Hello from tee test" | ./ex03_tee /tmp/tee_output.txt
	@echo "--- check file contents ---"
	@cat /tmp/tee_output.txt
	@echo "--- append mode ---"
	@echo "Second line" | ./ex03_tee -a /tmp/tee_output.txt
	@cat /tmp/tee_output.txt
	@rm /tmp/tee_output.txt
	@echo ""
	@echo "=== Testing ex04_bonus_cut ==="
	@echo "a,b,c,d,e" | ./ex04_bonus_cut -d , -f 1,3,5
	@echo "--- multiple fields ---"
	@echo "col1|col2|col3|col4" | ./ex04_bonus_cut -d '|' -f 2,4

# --- Project ---

project:
	cd project && cargo build && cargo test

project_test:
	cd project && cargo test

# --- Clean ---

clean:
	rm -f $(ALL_BINS)
	cd project && cargo clean 2>/dev/null || true
	rm -f contacts_data.json notebook_data.json records_input.txt records_report.txt
