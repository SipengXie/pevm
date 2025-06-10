//! Convert bincode data files to JSON format
//! 
//! This tool converts the binary-encoded block hashes and bytecodes to human-readable JSON format.

use std::{
    collections::BTreeMap,
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use alloy_primitives::{B256};
use clap::Parser;
use color_eyre::eyre::{Context, Result};
use flate2::bufread::GzDecoder;
use pevm::EvmCode;

#[derive(Parser, Debug)]
/// Convert bincode data files to JSON format
struct Args {
    /// Path to the data directory containing block_hashes.bincode and bytecodes.bincode.gz
    #[arg(short, long, default_value = "/home/blockchain/pevm/data")]
    data_dir: PathBuf,
    
    /// Output directory for JSON files
    #[arg(short, long, default_value = "/home/blockchain/pevm/json_output")]
    output_dir: PathBuf,
    
    /// Convert block hashes only
    #[arg(long)]
    block_hashes_only: bool,
    
    /// Convert bytecodes only  
    #[arg(long)]
    bytecodes_only: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let args = Args::parse();
    
    // Create output directory
    std::fs::create_dir_all(&args.output_dir)
        .context("Failed to create output directory")?;
    
    if !args.bytecodes_only {
        convert_block_hashes(&args.data_dir, &args.output_dir)?;
    }
    
    if !args.block_hashes_only {
        convert_bytecodes(&args.data_dir, &args.output_dir)?;
    }
    
    println!("Conversion completed successfully!");
    println!("JSON files saved to: {}", args.output_dir.display());
    
    Ok(())
}

fn convert_block_hashes(data_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
    println!("Converting block hashes...");
    
    let block_hashes_path = data_dir.join("block_hashes.bincode");
    let output_path = output_dir.join("block_hashes.json");
    
    // Read and deserialize block hashes
    let file = File::open(&block_hashes_path)
        .with_context(|| format!("Failed to open {}", block_hashes_path.display()))?;
    
    let block_hashes: BTreeMap<u64, B256> = bincode::deserialize_from::<_, BTreeMap<u64, B256>>(file)
        .context("Failed to deserialize block hashes")?;
    
    println!("Found {} block hashes", block_hashes.len());
    
    // Convert to JSON-friendly format (B256 to hex string)
    let json_data: BTreeMap<String, String> = block_hashes
        .into_iter()
        .map(|(block_num, hash)| (block_num.to_string(), format!("{:#x}", hash)))
        .collect();
    
    // Write to JSON file
    let output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create {}", output_path.display()))?;
    
    serde_json::to_writer_pretty(output_file, &json_data)
        .context("Failed to write block hashes JSON")?;
    
    println!("Block hashes saved to: {}", output_path.display());
    Ok(())
}

fn convert_bytecodes(data_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
    println!("Converting bytecodes...");
    
    let bytecodes_path = data_dir.join("bytecodes.bincode.gz");
    let output_path = output_dir.join("bytecodes.json");
    
    // Read and decompress bytecodes
    let compressed_file = File::open(&bytecodes_path)
        .with_context(|| format!("Failed to open {}", bytecodes_path.display()))?;
    
    let decoder = GzDecoder::new(BufReader::new(compressed_file));
    
    let bytecodes: BTreeMap<B256, EvmCode> = bincode::deserialize_from(decoder)
        .context("Failed to deserialize bytecodes")?;
    
    println!("Found {} bytecodes", bytecodes.len());
    
    // Convert to JSON-friendly format
    let json_data: BTreeMap<String, serde_json::Value> = bytecodes
        .into_iter()
        .map(|(hash, code)| {
            let code_json = serde_json::to_value(&code)
                .unwrap_or_else(|_| serde_json::Value::String("Failed to serialize code".to_string()));
            (format!("{:#x}", hash), code_json)
        })
        .collect();
    
    // Write to JSON file
    let output_file = File::create(&output_path)
        .with_context(|| format!("Failed to create {}", output_path.display()))?;
    
    serde_json::to_writer_pretty(output_file, &json_data)
        .context("Failed to write bytecodes JSON")?;
    
    println!("Bytecodes saved to: {}", output_path.display());
    Ok(())
} 