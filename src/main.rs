use std::path::Path;
use cairo_vm::types::layout_name::LayoutName;
use prove_block::prove_block;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider, Url};

#[tokio::main]
async fn main() {
    let compiled_os: &[u8] = include_bytes!("../build/os_latest.json");

    let snos_url = "http://0.0.0.0:9944";
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(snos_url).unwrap()));
    let block_number = provider.block_number().await.unwrap();

    eprintln!("block_number = {:#?}", block_number);

    // let (cairo_pie, snos_output) = prove_block(compiled_os, block_number, snos_url, LayoutName::all_cairo, false).await.unwrap();
    // cairo_pie.write_zip_file(Path::new("./cairo_pie.zip")).expect("failed to write");
}
