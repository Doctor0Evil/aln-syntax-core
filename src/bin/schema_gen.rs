//! ALN Schema Generator - Converts .aln schemas to Rust types
//!
//! This binary reads .aln schema files and generates corresponding
//! Rust structs, enums, and validation code for all schema families.

use aln_syntax_core::generator::{SchemaGenerator, GeneratorConfig};
use aln_syntax_core::error::AlnError;
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), AlnError> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: aln-schema-gen <schema-directory> [output-directory]");
        eprintln!("  schema-directory: Path to .aln schema files");
        eprintln!("  output-directory: Path for generated Rust code (default: src/generated)");
        std::process::exit(1);
    }

    let schema_dir = PathBuf::from(&args[1]);
    let output_dir = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from("src/generated")
    };

    println!("🔧 ALN Schema Generator v{}", aln_syntax_core::SCHEMA_VERSION);
    println!("📂 Schema directory: {}", schema_dir.display());
    println!("📁 Output directory: {}", output_dir.display());

    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    let config = GeneratorConfig {
        target_language: "rust".to_string(),
        include_validation: true,
        include_serde: true,
        include_docs: true,
        hex_stamp_verification: true,
    };

    let mut generator = SchemaGenerator::new(config);

    // Discover all .aln files in schema directory
    let schema_files = discover_aln_files(&schema_dir)?;
    println!("📄 Found {} schema files", schema_files.len());

    for schema_file in &schema_files {
        println!("🔄 Processing: {}", schema_file.display());
        generator.process_schema(schema_file, &output_dir)?;
    }

    // Generate mod.rs for module organization
    generator.generate_module_file(&output_dir)?;

    // Generate hex-stamp for this generation run
    let hex_stamp = generator.generate_hex_stamp()?;
    println!("✅ Hex-stamp: {}", hex_stamp);

    println!("✨ Schema generation complete!");
    println!("📋 Generated files in: {}", output_dir.display());

    Ok(())
}

fn discover_aln_files(dir: &PathBuf) -> Result<Vec<PathBuf>, AlnError> {
    let mut files = Vec::new();
    
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                files.extend(discover_aln_files(&path)?);
            } else if path.extension().map_or(false, |ext| ext == "aln") {
                files.push(path);
            }
        }
    }

    Ok(files)
}
