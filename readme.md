sncast --wait --profile account1 declare --url $RPC_URL --fee-token strk --contract-name test
stone-cli prove-bootloader --cairo_pies ./cairo_pie.zip --layout starknet-with-keccak --parameter_file ./tests/configs/bootloader_cpu_air_params_layout_starknet_with_keccak.json --output bootloader_proof.json --fact_topologies_output fact_topologies.json
