PROJECT_NAME := $(notdir $(subst %/,%,$(abspath .)))

export RUSTFLAGS := -C target-cpu=native
CARGO_VERBS := clippy run build fmt

$(foreach x,$(CARGO_VERBS),cargo-$x): cargo-%:
	cargo $* --release

$(CARGO_VERBS): %:
	cargo $* --release

cargo-clean clean:
	cargo clean

target/release/$(PROJECT_NAME): build

size: target/release/$(PROJECT_NAME)
	strip $<
	size $<
