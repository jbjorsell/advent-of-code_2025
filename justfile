run crate: 
	#!/usr/bin/env bash
	cargo run --bin {{crate}}

debug crate $RUST_LOG="DEBUG": 
    just run {{crate}}