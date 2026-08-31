use clap::Parser;
use kubelab_validation_engine::models::DeclarativeLabDef;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Validate KubeLab Declarative Lab YAML Schema")]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn validate_single_file(path: &Path) -> Result<DeclarativeLabDef, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Error reading lab file {:?}: {}", path, e))?;

    serde_yaml::from_str::<DeclarativeLabDef>(&content)
        .map_err(|e| format!("VALIDATION FAILED for {:?}: {}", path, e))
}

fn main() {
    let args = Args::parse();

    if args.path.is_dir() {
        let mut count = 0;
        let mut errors = Vec::new();

        for entry in WalkDir::new(&args.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let p = entry.path();
            if let Some(ext) = p.extension() {
                if ext == "yaml" || ext == "yml" {
                    match validate_single_file(p) {
                        Ok(lab) => {
                            count += 1;
                            println!(
                                "  [VALID] Lab '{}' (id: {}) with {} tasks",
                                lab.title,
                                lab.id,
                                lab.tasks.len()
                            );
                        }
                        Err(err) => {
                            errors.push(err);
                        }
                    }
                }
            }
        }

        if !errors.is_empty() {
            eprintln!("\nValidation failed with {} error(s):", errors.len());
            for err in errors {
                eprintln!("  {}", err);
            }
            std::process::exit(1);
        }

        println!(
            "\nSUCCESS: Successfully validated all {} declarative YAML lab definition(s) under {:?}.",
            count, args.path
        );
        std::process::exit(0);
    } else {
        match validate_single_file(&args.path) {
            Ok(lab) => {
                println!(
                    "SUCCESS: Lab '{}' (id: {}) with {} tasks is 100% valid schema.",
                    lab.title,
                    lab.id,
                    lab.tasks.len()
                );
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
