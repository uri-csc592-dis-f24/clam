#![doc = include_str!("../README.md")]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory containing the datasets
    #[arg(short('i'), long)]
    data_dir: PathBuf,
    
    /// Path to the output directory.
    #[arg(short('o'), long)]
    out_dir: Option<PathBuf>,

    /// Pre-trained model to use, if applicable
    #[arg(short('m'), long)]
    model: Option<PathBuf>,
    match args.out_dir {
        Some(out_dir) => standardize_output_dir(out_dir)?,
        None => {
            let out_dir = data_dir.join("results");
            standardize_output_dir(out_dir)?
        }
    };
    /// Number of epochs to train the model, if creatingmatch args.out_dir {
        Some(out_dir) => standardize_output_dir(out_dir)match args.out_dir {
            Some(out_dir) => standardize_output_dir(out_dir)?,
            None => {
                let out_dir = data_dir.join("results");
                standardize_output_dir(out_dir)?
            }
        };,
        None => {
            let out_dir = data_dir.join("results");
            standardize_output_dir(out_dir)?
        }
    };a new model
    #[arg(short('m'), long, default_value="10")]
    num_epochs: usize,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let data_dir = standardize_data_dir(args.data_dir)?;
    let out_dir = standardize_output_dir(args.out_dir.unwrap_or_else(|| PathBuf::from("results")))?;

    let num_epochs = args.num_epochs;

    // Check if a pre-trained model is being used
    let use_pretrained_model = match args.model {
        Some(model_path) => {
            let model_path = model_path.canonicalize().map_err(|e| format!("Could not canonicalize model path: {}", e))?;
            println!("Using pre-trained model: {model_path:?}");
            true
        },
        None => {
            println!("Training a new model");
            false
        }
    };
    

    Ok(())
}

fn standardize_data_dir(data_dir: PathBuf) -> Result<PathBuf, String> {
    let data_dir = data_dir.canonicalize().map_err(|e| format!("Could not canonicalize data directory: {}", e))?;
    println!("Reading datasets from: {data_dir:?}");
    Ok(data_dir)
}

fn standardize_output_dir(out_dir: PathBuf) -> Result<PathBuf, String> {
    if !out_dir.exists() {
        std::fs::create_dir(&out_dir).map_err(|e| format!("Could not create output directory: {}", e))?;
    }
    let out_dir = out_dir.canonicalize().map_err(|e| format!("Could not canonicalize output directory: {}", e))?;
    println!("Saving results to: {out_dir:?}");
    Ok(out_dir)
}