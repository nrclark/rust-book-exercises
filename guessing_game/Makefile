export RUSTFLAGS := -C target-cpu=native
CARGO_VERBS := clippy run build

$(foreach x,$(CARGO_VERBS),cargo-$x): cargo-%:
	cargo $* --release

$(CARGO_VERBS): %:
	cargo $* --release

clean:
	rm -rf target
