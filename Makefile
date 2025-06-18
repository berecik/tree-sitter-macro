.PHONY: all build test check fix example clean

# Default target
all: build test

# Build the project
build:
	@echo "Building project..."
	@cargo build --all-features

# Run tests
test:
	@echo "Running tests..."
	@./scripts/run_tests.sh

# Check code quality
check:
	@echo "Checking code quality..."
	@./scripts/check_code_quality.sh

# Fix code quality issues
fix:
	@echo "Fixing code quality issues..."
	@./scripts/fix_code_quality.sh

# Run the example
example:
	@echo "Running example..."
	@cargo run --example parse_c --features proc_macros

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# Help target
help:
	@echo "Available targets:"
	@echo "  all      - Build the project and run tests (default)"
	@echo "  build    - Build the project"
	@echo "  test     - Run tests and examples"
	@echo "  check    - Check code quality"
	@echo "  fix      - Fix code quality issues"
	@echo "  example  - Run the example"
	@echo "  clean    - Clean build artifacts"
	@echo "  help     - Show this help message"