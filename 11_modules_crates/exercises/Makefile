# Exercise Makefile — compile and verify all exercise solutions
#
# Targets:
#   check    — compile all exercises (default)
#   run      — run all exercises
#   clean    — remove compiled binaries

ROOT = ..

EXERCISES = ex01_greeter ex02_calculator_modules \
            ex03_library_vs_binary ex04_pub_use_export

# Solutions have corresponding _sol.rs files
SOLUTIONS = ex01_greeter_sol ex02_calculator_modules_sol \
            ex03_library_vs_binary_sol ex04_pub_use_export_sol

# All concept files (standalone)
CONCEPTS_SINGLE = 02_modules_basics 05_pub_use 06_use_paths 07_cargo_basics 08_workspaces

.PHONY: all check run clean

all: check

check:
	@echo "=== Checking concept files ==="
	@for f in $(CONCEPTS_SINGLE); do \
		echo "  Compiling $(ROOT)/concept/$$f.rs ..."; \
		cd $(ROOT)/concept && rustc --edition 2021 $$f.rs -o /tmp/$$f && echo "  OK"; \
	done
	@echo "  Compiling $(ROOT)/concept/01_multiple_files.rs (finds util.rs)..."
	@cd $(ROOT)/concept && rustc --edition 2021 01_multiple_files.rs -o /tmp/01_multiple_files && echo "  OK"
	@echo "  Compiling $(ROOT)/concept/03_modules_file.rs (finds greetings.rs)..."
	@cd $(ROOT)/concept && rustc --edition 2021 03_modules_file.rs -o /tmp/03_modules_file && echo "  OK"
	@echo "  Compiling $(ROOT)/concept/04_modules_subdir.rs (with utils/ subdir)..."
	@cd $(ROOT)/concept && rustc --edition 2021 04_modules_subdir.rs -o /tmp/04_modules_subdir && echo "  OK"
	@echo "=== Checking exercise solutions ==="
	@for f in $(SOLUTIONS); do \
		if [ -f $$f.rs ]; then \
			echo "  Compiling $$f.rs ..."; \
			if echo "$$f" | grep -q "ex03_library"; then \
				rustc --edition 2021 $$f.rs ex03_library.rs -o /tmp/$$f && echo "  OK"; \
			else \
				rustc --edition 2021 $$f.rs -o /tmp/$$f && echo "  OK"; \
			fi \
		fi \
	done
	@echo "=== Checking project ==="
	@if [ -d $(ROOT)/project/file_organizer ]; then \
		cd $(ROOT)/project/file_organizer && cargo check 2>&1; \
	fi
	@echo "=== All checks passed ==="

run:
	@echo "=== Running concept files ==="
	@for f in $(CONCEPTS_SINGLE); do \
		echo "--- $(ROOT)/concept/$$f.rs ---"; \
		cd $(ROOT)/concept && rustc --edition 2021 $$f.rs -o /tmp/$$f && /tmp/$$f; \
	done
	@echo "--- 01_multiple_files ---"
	@cd $(ROOT)/concept && rustc --edition 2021 01_multiple_files.rs -o /tmp/01_multiple_files && /tmp/01_multiple_files
	@echo "--- 03_modules_file ---"
	@cd $(ROOT)/concept && rustc --edition 2021 03_modules_file.rs -o /tmp/03_modules_file && /tmp/03_modules_file
	@echo "--- 04_modules_subdir ---"
	@cd $(ROOT)/concept && rustc --edition 2021 04_modules_subdir.rs -o /tmp/04_modules_subdir && /tmp/04_modules_subdir
	@echo "=== Running exercise solutions ==="
	@for f in $(SOLUTIONS); do \
		if [ -f $$f.rs ]; then \
			echo "--- $$f.rs ---"; \
			if echo "$$f" | grep -q "ex03_library"; then \
				rustc --edition 2021 $$f.rs ex03_library.rs -o /tmp/$$f && /tmp/$$f; \
			else \
				rustc --edition 2021 $$f.rs -o /tmp/$$f && /tmp/$$f; \
			fi \
		fi \
	done
	@echo "=== Running project ==="
	@if [ -d $(ROOT)/project/file_organizer ]; then \
		cd $(ROOT)/project/file_organizer && cargo run -- --help 2>&1; \
	fi

clean:
	rm -f /tmp/01_multiple_files /tmp/02_modules_basics /tmp/03_modules_file /tmp/04_modules_subdir
	rm -f /tmp/05_pub_use /tmp/06_use_paths /tmp/07_cargo_basics /tmp/08_workspaces
	rm -f /tmp/ex01_greeter /tmp/ex01_greeter_sol
	rm -f /tmp/ex02_calculator_modules /tmp/ex02_calculator_modules_sol
	rm -f /tmp/ex03_library_vs_binary /tmp/ex03_library_vs_binary_sol
	rm -f /tmp/ex04_pub_use_export /tmp/ex04_pub_use_export_sol
	rm -rf $(ROOT)/project/file_organizer/target
	@echo "Cleaned."
