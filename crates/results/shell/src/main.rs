//Ignore unused variables for now
#![allow(unused_variables)]
#![doc = include_str!("../README.md")]

use clap::Parser;
use std::path::PathBuf;

pub struct Shell {
    data_dir: PathBuf,
    out_dir: PathBuf,
    num_epochs: usize,
    min_depth: usize,
    model: PathBuf,
    seed: u64,
}

impl Shell {
    /// Create a new Shell object from the provided arguments
    pub fn new(args: Args, seed: u64) -> Result<Self, String> {
        let num_epochs = args.epochs;
        let min_depth = args.min_depth;

        // Standardize the data directory
        let data_dir = standardize_data_dir(args.data_dir)?;

        // Standardize the output directory. If none provided, default to "results"
        let out_dir = standardize_output_dir(args.out_dir.unwrap_or_else(|| PathBuf::from("results")))?;

        // Get the model to use
        let model = get_model(args.model)?;

        Ok(Self {
            data_dir,
            out_dir,
            num_epochs,
            min_depth,
            model,
            seed,
        })
    }

    /// Print the properties of the Shell object
    pub fn print(&self) {
        println!("Reading datasets from: {:?}", self.data_dir);
        println!("Saving results to: {:?}", self.out_dir);
        println!("Training model for {} epochs", self.num_epochs);
        println!("Minimum depth of clusters: {}", self.min_depth);
        println!("Model path: {:?}", self.model);
        println!("Seed: {}", self.seed);
    }

    /// Write the trained model to disk
    pub fn write_to_disk(&self) {
        let todo = "Serialize the model and write it to disk";

        println!("Writing model to {:?}", self.model);
    }

    /// Train the model
    pub fn train(&self) {
        let todo = "Train the model";
        
        for epoch in 0..self.num_epochs {
            println!("Training epoch {}/{}", epoch + 1, self.num_epochs);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    /// Test the model
    pub fn test(&self) {
        let todo = "Test the model";

        println!("Testing model...");
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("ROC AUC: 0.95");
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
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
    epochs: usize,

    /// Minimum depth of the clusters used for making graphs
    #[arg(short('d'), long, default_value="4")]
    min_depth: usize,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    // Build the shell model using provided arguments
    let shell = Shell::new(args, 1729)?;

    // Train the model
    shell.train();

    // Save the model
    shell.write_to_disk();

    // Test the model
    shell.test();

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