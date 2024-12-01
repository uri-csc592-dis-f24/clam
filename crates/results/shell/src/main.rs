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

    /// Number of epochs to train the model, if creating a new model
    #[arg(short('m'), long, default_value="10")]
    num_epochs: usize,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let num_epochs = args.num_epochs;

    // Standardize the data directory
    let data_dir = standardize_data_dir(args.data_dir)?;

    // Standardize the output directory. If none provided, default to "results"
    let out_dir = standardize_output_dir(args.out_dir.unwrap_or_else(|| PathBuf::from("results")))?;

    // Get the model to use
    let model = get_model(args.model)?;

    // Set a seed for tree generation
    let seed = 1729;

    print_parameters(&data_dir, &out_dir, num_epochs, model, seed);

    Ok(())
}

fn standardize_data_dir(data_dir: PathBuf) -> Result<PathBuf, String> {
    let data_dir = data_dir.canonicalize().map_err(|e| format!("Could not canonicalize data directory: {}", e))?;
    Ok(data_dir)
}

fn standardize_output_dir(out_dir: PathBuf) -> Result<PathBuf, String> {
    // Create the output directory if it doesn't exist
    if !out_dir.exists() {
        println!("Output directory does not exist. Creating: {out_dir:?}");
        std::fs::create_dir(&out_dir).map_err(|e| format!("Could not create output directory: {}", e))?;
    }

    let out_dir = out_dir.canonicalize().map_err(|e| format!("Could not canonicalize output directory: {}", e))?;
    Ok(out_dir)
}

/// If model path is provided and exists, return its canonicalized path. Otherwise, return a new filename to use
/// for the model. This will be changed in the future to create a Shell object in abd_clam for use here
/// TODO: Create a Shell object in abd_clam for use here
fn get_model(model_path: Option<PathBuf>) -> Result<PathBuf, String> {
    let todo = "Create a Shell object in abd_clam for use here";
    match model_path {
        Some(path) => {
            // If the model doesn't exist, create a new model
            let todo = "Create a new model if it doesn't exist";

            if !path.exists() {
                println!("Model path provided does not exist. Creating new model: {path:?}");
            }

            let model_path = path.canonicalize().map_err(|e| format!("Could not canonicalize model path: {}", e))?;
            Ok(model_path)
        },
        None => {
            let model_path = PathBuf::from("shell.bin");
            Ok(model_path)
        }
    }
}

fn print_parameters(data_dir: &PathBuf, out_dir: &PathBuf, num_epochs: usize, model: PathBuf, seed: u64) {
    println!("Reading datasets from: {data_dir:?}");
    println!("Saving results to: {out_dir:?}");
    println!("Training model for {num_epochs} epochs");
    println!("Model path: {model:?}");
    println!("Seed: {seed}");
}