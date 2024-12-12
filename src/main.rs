use std::env;
use std::path::Path;
use cairo_vm::types::layout_name::LayoutName;
use prove_block::prove_block;

#[tokio::main]
async fn main() {
    let compiled_os: &[u8] = include_bytes!("../build/os-0.13.1.json");
    let snos_url = "http://0.0.0.0:9545";

    let args: Vec<String> = env::args().collect();

    let block_number = &args[1];
    let layout = LayoutName::all_cairo;

    let (cairo_pie, snos_output) = prove_block(compiled_os, block_number.parse().unwrap(), snos_url, layout, false).await.unwrap();

    cairo_pie.write_zip_file(Path::new(format!("./output/cairo-pie/{}-{}-cairo-pie.zip", block_number, layout).as_str())).expect("failed to write");
    std::fs::write(format!("./output/snos-output/{}-{}-output.json", block_number, layout), serde_json::to_string_pretty(&snos_output).unwrap().as_bytes()).unwrap()
}
