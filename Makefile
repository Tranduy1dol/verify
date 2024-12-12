run-madara:
	cd madara && \
	git checkout 0f0be5666b1388172311b1d098c0e3120846aedc && \
	cargo run --release -- --name madara --base-path ../madara-db --rpc-port 9944 --rpc-cors "*" --rpc-external --devnet --preset devnet --feeder-gateway-enable --gateway-enable --gateway-external --no-l1-sync --gas-price 1 --blob-gas-price 1 --strk-gas-price 1 --strk-blob-gas-price 1
run-pathfinder:
	cd pathfinder && \
	git checkout e37f5431 && \
	cargo run --release --bin pathfinder -- --network custom --chain-id MADARA_DEVNET --feeder-gateway-url http://localhost:8080/feeder_gateway --gateway-url http://localhost:8080/gateway --storage.state-tries archive --data-directory ../pathfinder-db
clean:
	sudo rm -r ./madara-db && \
	sudo rm -r ./pathfinder-db
gen-os-v0.13.1:
	cd cairo-lang && \
	git checkout ab6d079f && \
	cd .. && \
	pip install cairo-lang==0.13.1 "sympy<1.13.0" && \
	cairo-compile cairo-lang/src/starkware/starknet/core/os/os.cairo --output build/os-0.13.1.json --cairo_path cairo-lang/src
gen-os-v0.13.2:
	cd cairo-lang && \
	git checkout a86e92bfde9c171c0856d7b46580c66e004922f3 && \
	cd .. && \
	pip install cairo-lang==0.13.2 "sympy<1.13.0" && \
	cairo-compile cairo-lang/src/starkware/starknet/core/os/os.cairo --output build/os-0.13.2.json --cairo_path cairo-lang/src
txs:
	export RUST_LOG="" && \
	cd contract && \
	scarb run spawn
run:
	cargo +nightly run -- 11