# Derived from https://github.com/juspay/hyperswitch/blob/main/Makefile
# = Parameters
# Override envars using -e

#
# = Common
#

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)

#
# = Targets
#

.PHONY : \
	init-repo \
	migrate-run \
	migrate-create \
	prepare \
	prepare-check \
	doc \
	fmt \
	clippy \
	test \
	audit \
	git.sync \
	build \
	push \
	shell \
	run \
	start \
	stop \
	rm \
	release \
	coverage

init-repo:
	cargo install --version="~0.8" sqlx-cli --no-default-features --features rustls,postgres
	./cpast_api/scripts/init_db.sh
	./cpast_api/scripts/init_redis.sh

# Compile application for running on local machine
#
# Usage :
#	make build

build :
	cargo build

# Generate crates documentation from Rust sources.
#
# Usage :
#	make doc [private=(yes|no)] [open=(yes|no)] [clean=(no|yes)]

doc :
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	SQLX_OFFLINE=true cargo doc --all-features --no-deps\
		$(if $(call eq,$(private),no),,--document-private-items) \
		$(if $(call eq,$(open),no),,--open)

# Format Rust sources with rustfmt.
#
# Usage :
#	make fmt [fix=(no|yes)]

fmt :
	cargo fmt --all $(if $(call eq,$(fix),yes),,-- --check)

# Lint Rust sources with Clippy.
#
# Usage :
#	make clippy

clippy :
	cargo clippy --all-features --all-targets -- -D warnings

# Run Rust tests of project.
#
# Usage :
#	make test

test :
	cargo test --all-features

# Usage
# make migrate-run new_fancy_table
migrate-create:
	cd cpast_api && cargo sqlx migrate add -r $(1)

migrate-run:
	cd cpast_api && cargo sqlx migrate run

prepare:
	cargo sqlx prepare --workspace -- --all-targets

prepare-check:
	cargo sqlx prepare --workspace --check -- --all-targets
# Run format clippy test and tests.
#
# Usage :
#	make precommit

coverage:
	cargo llvm-cov clean --workspace --html --output-dir=coverage
	cargo llvm-cov --all-features --workspace --no-clean --html --output-dir=coverage --open
	
precommit : fmt clippy test prepare-check
