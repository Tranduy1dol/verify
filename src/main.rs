use std::path::Path;
use cairo_vm::types::layout_name::LayoutName;
use prove_block::prove_block;

#[tokio::main]
async fn main() {
    let compiled_os: &[u8] = include_bytes!("../build/os_latest.json");

    let snos_url = "http://0.0.0.0:9545";
    let block_number = 13;

    let (cairo_pie, snos_output) = prove_block(compiled_os, block_number, snos_url, LayoutName::starknet, false).await.unwrap();
    cairo_pie.write_zip_file(Path::new("./cairo_pie.zip")).expect("failed to write");
}
