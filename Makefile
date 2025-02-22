.PHONY: clippy-fixes clippy-strict

all: lint

lint:
	cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used -A clippy::module_name_repetitions -A clippy::needless_pass_by_value

run:
	cargo run --features bevy/dynamic_linking

test:
	cargo test

debug-test:
	#cargo test -- --nocapture
	cargo test integration::k8s_live::k8s_api::tests::test_get_names_pods -- --nocapture

build:
	cargo build

clean:
	cargo clean
