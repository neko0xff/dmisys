CC1:=cargo
CC2:=rustup
CC3:=cargo-clippy

.PHONY: build run clean  build_release run_release format format-check lint test version test_bin

all: version clean run_release 

format:
	@$(CC1)  fmt --quiet

format-check:
	@$(CC2)  component add rustfmt 2> /dev/null
	@$(CC1)  fmt --all -- --check

lint:
	@$(CC2)  component add clippy 2> /dev/null
	@$(CC3) --all-targets --all-features -- -D warnings 

test:
	@$(CC1) test
	
test_bin:	
	./target/release/example	

build_release:
	@$(CC1) build	--release

run_release: 
	@$(CC1) run	--release

build_debug:
	@$(CC1) build

run_debug:
	@$(CC1) run

clean:
	@$(CC1) clean	

version:
	@echo ""
	@echo "Rust CLI Versions:"
	@echo "------------------"
	@echo "rustc (compiler):  $$(rustc --version | sed 's/rustc //' | tr -d '\n')"
	@echo "cargo (package manager): $$(cargo --version | sed 's/cargo //' | tr -d '\n')"
	@echo "rustfmt (code formatter):  $$(rustfmt --version | sed 's/rustfmt //' | tr -d '\n')"
	@echo "clippy (linter):  $$(clippy-driver --version | sed 's/clippy //' | tr -d '\n')"
	@echo "rustup (toolchain manager): $$(rustup --version 2>/dev/null | sed 's/rustup //' | tr -d '\n')"
	@echo ""