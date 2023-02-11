# Installs the program
install:
	cargo install

# Runs the tests
test:
	cargo nextest run

# Analyzes the program and reports errors
check:
	cargo check

build:
	cargo build

update:
	cargo update

