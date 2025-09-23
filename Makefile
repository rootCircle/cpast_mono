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
	bench \
	clippy \
	test \
	nextest \
	audit \
	git.sync \
	build \
	release-cli \
	push \
	shell \
	run \
	start \
	stop \
	rm \
	release \
	coverage \
	precommit \
	precommit-fix \
	example_test \
	example_generate \
	example_test_multilang \


init-repo:
	@echo "Ensure you have GOOGLE_API_KEY in the env before you run this!"
	cargo install --version="~0.8" sqlx-cli --no-default-features --features rustls,postgres
	./cpast_api/scripts/init_db.sh
	./cpast_api/scripts/init_redis.sh
	python scripts/secrets_for_ci.py
	git update-index --assume-unchanged ./cpast_api/configuration/base.yaml

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

bench:
	cargo bench --all

# Run Rust tests of project.
#
# Usage :
#	make test

test :
	cargo test --all-features

nextest:
	cargo nextest run --all-features --all-targets

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

release-cli:
	cargo smart-release cpast --execute --update-crates-index

coverage:
	cargo llvm-cov clean --workspace --html --output-dir=coverage
	cargo llvm-cov --all-features --workspace --no-clean --html --output-dir=coverage --open

example_test:
	cargo run --example=test

example_generate:
	cargo run --example=generate 

example_test_multilang:
	cargo run --example=test_multilang

precommit : fmt clippy prepare-check bench example_test example_generate example_test_multilang nextest

precommit-fix:
	cargo clippy --all-features --all-targets --fix --allow-dirty
	cargo fmt --all
