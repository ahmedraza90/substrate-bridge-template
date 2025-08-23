./target/release/solochain-template-node --dev --rpc-port 9944 --rpc-external --port 30334

subxt codegen --url ws://localhost:9944 > chain_a_codegen_runtime.rs

./target/release/solochain-template-node --dev --rpc-port 9945 --rpc-external --port 30333

subxt codegen --url ws://localhost:9945 > chain_a_codegen_runtime.rs


subxt metadata --url ws://localhost:9944 --format bytes > metadata.scale