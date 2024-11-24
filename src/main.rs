pub mod fact_info;
pub mod fact_node;
pub mod fact_topology;
pub mod error;
pub mod utils;

use std::io::Read;
use bytes::Bytes;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use prove_block::prove_block;
use tempfile::NamedTempFile;
use color_eyre::Result;
use crate::fact_info::get_fact_info;

#[tokio::main]
async fn main() {
    let compiled_os: &[u8] = include_bytes!("../build/os_latest.json");
    let snos_url = "";

    let (cairo_pie, snos_output) = prove_block(compiled_os, 15u64, snos_url, LayoutName::starknet, false).expect("Failed to run snos");
    let fact_info = get_fact_info(&cairo_pie, None)?;
    let program_output = fact_info.program_output;

    let cairo_pie_key = format!("{block_number}/{CAIRO_PIE_FILE_NAME}");
    let cairo_pie_zip_bytes = cairo_pie_to_zip_bytes(cairo_pie).await.expect("");

    let snos_output_key = format!("{block_number}/{SNOS_OUTPUT_FILE_NAME}");
    let snos_output_json = serde_json::to_vec(&snos_output).expect("");

    let program_output: Vec<[u8; 32]> = program_output.iter().map(|f| f.to_bytes_be()).collect();
    let encoded_data = bincode::serialize(&program_output).expect("");
}

fn tempfile_to_bytes(tmp_file: &mut NamedTempFile) -> Result<Bytes> {
    let mut buffer = Vec::new();
    tmp_file.as_file_mut().read_to_end(&mut buffer)?;
    Ok(Bytes::from(buffer))
}

async fn cairo_pie_to_zip_bytes(cairo_pie: CairoPie) -> Result<Bytes> {
    let mut cairo_pie_zipfile = NamedTempFile::new()?;
    cairo_pie.write_zip_file(cairo_pie_zipfile.path())?;
    let cairo_pie_zip_bytes = tempfile_to_bytes(&mut cairo_pie_zipfile)?;
    cairo_pie_zipfile.close()?;
    Ok(cairo_pie_zip_bytes)
}