use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Represents the result of OCR detection for a single text region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// Bounding box coordinates as [[x1, y1], [x2, y2], [x3, y3], [x4, y4]]
    #[serde(default)]
    pub bbox: Vec<Vec<i32>>,
    /// Detected text
    pub text: String,
    /// Confidence score (0.0 to 1.0)
    #[serde(default)]
    pub confidence: f64,
}

/// RustOCR - A fast Rust CLI for EasyOCR with 80+ language support
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image file path
    #[arg(short, long)]
    input: String,

    /// Languages to recognize (comma-separated, e.g., "en" or "ch_sim,en")
    #[arg(short, long, default_value = "en", value_delimiter = ',')]
    languages: Vec<String>,

    /// Enable GPU acceleration (requires CUDA)
    #[arg(short, long, default_value = "true")]
    gpu: bool,

    /// Detail level: 0 for text only, 1 for bounding boxes and confidence
    #[arg(short, long, default_value = "1")]
    detail: i32,

    /// Output format: json, text, or detailed
    #[arg(short, long, default_value = "json")]
    output: String,
}

fn get_bridge_script_path() -> Result<PathBuf> {
    // Get the directory of the current executable
    let exe_path = std::env::current_exe()
        .context("Failed to get executable path")?;
    let exe_dir = exe_path.parent()
        .context("Failed to get executable directory")?;
    
    // Look for the bridge script in the same directory as the executable
    let bridge_script = exe_dir.join("easyocr_bridge.py");
    
    // If not found there, try the current working directory
    if !bridge_script.exists() {
        let cwd_bridge = PathBuf::from("easyocr_bridge.py");
        if cwd_bridge.exists() {
            return Ok(cwd_bridge);
        }
        
        // Try relative to current directory
        let relative_bridge = PathBuf::from("./easyocr_bridge.py");
        if relative_bridge.exists() {
            return Ok(relative_bridge);
        }
    }
    
    Ok(bridge_script)
}

fn run_ocr(
    image_path: &str,
    languages: &[String],
    gpu: bool,
    detail: i32,
) -> Result<Vec<OcrResult>> {
    let bridge_script = get_bridge_script_path()
        .context("Failed to locate easyocr_bridge.py")?;
    
    if !bridge_script.exists() {
        anyhow::bail!(
            "Bridge script not found at: {}. Make sure easyocr_bridge.py is in the same directory as the binary.",
            bridge_script.display()
        );
    }
    
    // Build the command
    let langs = languages.join(",");
    let output = Command::new("python3")
        .arg(&bridge_script)
        .arg("--languages")
        .arg(&langs)
        .arg("--image")
        .arg(image_path)
        .arg("--gpu")
        .arg(gpu.to_string())
        .arg("--detail")
        .arg(detail.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("Failed to execute Python bridge script")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("OCR processing failed: {}", stderr);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let results: Vec<OcrResult> = serde_json::from_str(&stdout)
        .context("Failed to parse OCR results from Python bridge")?;
    
    Ok(results)
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate input file exists
    if !Path::new(&args.input).exists() {
        anyhow::bail!("Input file does not exist: {}", args.input);
    }

    // Validate detail level
    if args.detail != 0 && args.detail != 1 {
        anyhow::bail!("Detail level must be 0 or 1");
    }

    // Print initialization message
    eprintln!("Initializing OCR with languages: {:?}", args.languages);
    eprintln!("GPU enabled: {}", args.gpu);
    eprintln!("Processing image: {}", args.input);

    // Perform OCR
    let results = run_ocr(&args.input, &args.languages, args.gpu, args.detail)
        .context("Failed to perform OCR")?;

    // Output results based on format
    match args.output.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&results)
                .context("Failed to serialize results to JSON")?;
            println!("{}", json);
        }
        "text" => {
            for result in &results {
                println!("{}", result.text);
            }
        }
        "detailed" => {
            for (i, result) in results.iter().enumerate() {
                println!("--- Result {} ---", i + 1);
                println!("Text: {}", result.text);
                if args.detail == 1 && result.confidence > 0.0 {
                    println!("Confidence: {:.4}", result.confidence);
                    if !result.bbox.is_empty() {
                        println!("Bounding Box: {:?}", result.bbox);
                    }
                }
                println!();
            }
        }
        _ => {
            anyhow::bail!("Invalid output format. Use: json, text, or detailed");
        }
    }

    eprintln!("OCR completed. Found {} text region(s).", results.len());

    Ok(())
}
