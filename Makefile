# rc — quality and build targets.
#
# `check` is the local gate. `deny`/`audit`/`machete`/`typos` are supply-chain
# and hygiene checks that skip gracefully when the tool is unavailable.

CARGO ?= cargo

.PHONY: check fmt clippy clippy-strict clippy-warning-count test determinism build file-length clean deny audit machete typos bench bench-quick

## Full local gate: formatting, lints, tests, determinism and the file-length budget.
check: fmt clippy test determinism file-length

## Check formatting without modifying files.
fmt:
	$(CARGO) fmt --all -- --check

## Lints for every target. Critical lints are `deny` in [workspace.lints];
## the remaining pedantic/nursery findings are a visible ratchet, not a hard gate.
clippy:
	$(CARGO) clippy --workspace --all-targets

## Strict clippy gate. This is the target CI will require once the ratchet is
## paid down; local `clippy` remains the visible-work target until then.
clippy-strict:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

## Diagnostic inventory for the clippy paydown. Not a gate.
clippy-warning-count:
	./scripts/clippy-warning-count.sh

## Workspace tests (includes doc-tests).
test:
	$(CARGO) test --workspace

## Byte-for-byte determinism contract tests.
determinism:
	$(CARGO) test -p rustfs-cli --test determinism_contract

## Debug build of the workspace.
build:
	$(CARGO) build --workspace

## No production Rust file may exceed 500 lines unless it is generated/fixture data.
file-length:
	./scripts/check-file-length.sh

clean:
	$(CARGO) clean

## Supply chain: advisories, licenses, bans and sources.
deny:
	@if command -v cargo-deny >/dev/null 2>&1; then \
		$(CARGO) deny check; \
	else \
		echo "cargo-deny absent; skipping (cargo install cargo-deny --locked)"; \
	fi

audit:
	@if command -v cargo-audit >/dev/null 2>&1; then \
		$(CARGO) audit; \
	else \
		echo "cargo-audit absent; skipping (cargo install cargo-audit --locked)"; \
	fi

machete:
	@if command -v cargo-machete >/dev/null 2>&1; then \
		$(CARGO) machete crates/cli crates/core crates/s3; \
	else \
		echo "cargo-machete absent; skipping (cargo install cargo-machete --locked)"; \
	fi

typos:
	@if command -v typos >/dev/null 2>&1; then \
		typos; \
	else \
		echo "typos absent; skipping (https://github.com/crate-ci/typos)"; \
	fi

## Performance bench + A/B comparator (outside the workspace; observational).
bench:
	$(CARGO) build --release -p rustfs-cli
	$(CARGO) run --release --manifest-path bench/Cargo.toml -- all \
		--rc target/release/rc --samples 15 \
		--out bench/results/latest.md --json bench/results/latest.json

bench-quick:
	$(CARGO) build --release -p rustfs-cli
	$(CARGO) run --release --manifest-path bench/Cargo.toml -- all \
		--rc target/release/rc --samples 5 --out bench/results/quick.md --json bench/results/quick.json
